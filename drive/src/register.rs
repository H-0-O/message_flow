//! Code generation for handler registration
//!
//! This module generates the `Register` trait implementation that sets up subscriptions
//! for all message handlers defined in a `#[msg_flow]` impl block.

use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_str, ImplItem, ItemImpl};

use crate::{
    attribute::AttributeType,
    constants::REGISTER_TRAIT_PATH,
    error::GeneratorResult,
    handler::{has_context_parameter, has_json_value_parameter, has_self_parameter},
    MsgFlowArgs,
};

/// Represents the type of handler (Message or Event)
///
/// This enum distinguishes between request-reply message handlers and fire-and-forget
/// event handlers. Currently, only `Message` handlers are fully implemented.
#[derive(Debug, Clone, Copy)]
pub enum HandlerType {
    /// Request-reply message handler (sends a response)
    Message,
    /// Fire-and-forget event handler (no response)
    Event,
}

/// Metadata about a handler function
///
/// This struct stores information about a handler function that is used during
/// code generation to create the appropriate subscription and handler code.
#[derive(Debug)]
pub struct HandlerMetadata {
    /// Whether this is a message or event handler
    pub handler_type: HandlerType,
    /// The name of the handler function as a token stream
    pub function_name: proc_macro2::TokenStream,
    /// Whether the handler function accepts a `Context` parameter
    pub has_context: bool,
    /// Whether the handler function accepts a `serde_json::Value` parameter
    pub has_json_value: bool,
    /// Whether the handler function has a `&self` parameter
    pub has_self: bool,
}

/// Collect all handlers from an impl block
///
/// Returns a map from pattern to handler metadata.
/// Returns an error if:
/// - Multiple handlers have the same pattern
/// - An attribute cannot be parsed
pub fn collect_handlers(
    impl_block: &ItemImpl,
) -> Result<HashMap<String, HandlerMetadata>, syn::Error> {
    let mut handlers: HashMap<String, HandlerMetadata> = HashMap::new();

    for item in &impl_block.items {
        if let ImplItem::Fn(func) = item {
            let func_name = &func.sig.ident;
            let has_ctx = has_context_parameter(&func.sig.inputs);
            let has_json_value = has_json_value_parameter(&func.sig.inputs);
            let has_self = has_self_parameter(&func.sig.inputs);

            for attr in &func.attrs {
                let attribute_type = match AttributeType::from_attribute(attr) {
                    Ok(attr_type) => attr_type,
                    Err(err) => return Err(err),
                };

                let pattern = attribute_type.pattern().to_string();

                // Validate: ensure no duplicate patterns
                if handlers.contains_key(&pattern) {
                    return Err(syn::Error::new_spanned(
                        attr,
                        format!(
                            "Duplicate pattern '{}'. Each pattern must be unique within an impl block.",
                            pattern
                        ),
                    ));
                }

                let handler_type = if attribute_type.is_message() {
                    HandlerType::Message
                } else if attribute_type.is_event() {
                    HandlerType::Event
                } else {
                    // This should never happen due to AttributeType::from_attribute validation
                    unreachable!("Unknown attribute type")
                };

                handlers.insert(
                    pattern.clone(),
                    HandlerMetadata {
                        handler_type,
                        function_name: quote! { #func_name },
                        has_context: has_ctx,
                        has_json_value,
                        has_self,
                    },
                );
            }
        }
    }

    Ok(handlers)
}

/// Generate the implementation of the Register trait
///
/// This function generates the complete `Register` trait implementation that:
/// 1. Collects all handlers from the impl block
/// 2. Creates subscriptions for each handler pattern
/// 3. Spawns tasks to handle incoming messages concurrently
///
/// # Arguments
///
/// * `struct_name` - The name of the struct being implemented
/// * `impl_block` - The impl block containing handler methods
/// * `args` - The parsed `#[msg_flow]` attribute arguments
///
/// # Returns
///
/// Returns a `TokenStream` containing the generated `Register` trait implementation.
///
/// # Errors
///
/// Returns an error if:
/// - The `Register` trait path cannot be parsed
/// - Handler collection fails (e.g., duplicate patterns)
///
/// # Generated Code Structure
///
/// The generated code:
/// 1. Implements `Register` for the struct
/// 2. For each handler, creates a subscription and spawns a task
/// 3. Each task deserializes incoming messages and calls the appropriate handler
/// 4. Handles errors and sends responses back to the client
pub fn generate_register_impl(
    struct_name: &syn::Ident,
    impl_block: &ItemImpl,
    args: &MsgFlowArgs,
) -> GeneratorResult {
    let register_trait_path: syn::Path = parse_str(REGISTER_TRAIT_PATH)?;

    let base_pattern = match &args.pattern {
        Some(pattern) => format!("{}.", pattern),
        None => String::new(),
    };

    let handlers = collect_handlers(impl_block)?;

    let subscription_tokens: Vec<TokenStream> = handlers
        .iter()
        .map(|(pattern, metadata)| {
            generate_subscription_code(struct_name, pattern, metadata, &base_pattern)
        })
        .collect();

    Ok(quote! {
        #[automatically_derived]
        #[message_flow::async_trait]
        impl #register_trait_path for #struct_name {
            async fn register(context: std::sync::Arc<message_flow::Context>) -> message_flow::Result<()> {
                #(#subscription_tokens)*
                Ok(())
            }
        }
    })
}

/// Generate code for a single subscription/handler
///
/// This function generates the code for subscribing to a message pattern and
/// handling incoming messages. It creates a spawned task that:
/// 1. Subscribes to the topic pattern
/// 2. Loops over incoming messages
/// 3. Deserializes the message payload
/// 4. Calls the appropriate handler function
/// 5. Sends the response back (for message handlers)
///
/// # Arguments
///
/// * `struct_name` - The name of the struct containing the handler
/// * `pattern` - The message pattern for this handler
/// * `metadata` - Metadata about the handler function
/// * `base_pattern` - The base pattern from `#[msg_flow(pattern = "...")]`
///
/// # Returns
///
/// Returns a `TokenStream` containing the subscription and handler code.
fn generate_subscription_code(
    struct_name: &syn::Ident,
    pattern: &str,
    metadata: &HandlerMetadata,
    base_pattern: &str,
) -> TokenStream {
    let topic_pattern = format!("{}{}", base_pattern, pattern);
    let function_name = &metadata.function_name;

    let reply_block = generate_reply_block(struct_name);

    // Generate handler code based on the detected handler signature
    // - `&self` present  → static schema (deserialize into struct and call method)
    // - `serde_json::Value` present → dynamic schema (pass raw JSON value)
    let handler_code = match metadata.handler_type {
        HandlerType::Message => {
            generate_message_handler_conditional(struct_name, function_name, metadata, &reply_block)
        }
        HandlerType::Event => {
            generate_event_handler_conditional(struct_name, function_name, metadata)
        }
    };

    quote! {
        tokio::spawn({
            let context = context.clone();
            async move {
                let client = &context.client;
                let mut subscription = client.subscribe(#topic_pattern).await?;

                message_flow::logger::info_log!(
                    "Spawned task for struct {}",
                    stringify!(#struct_name)
                );

                while let Some(msg) = subscription.next().await {
                    message_flow::logger::info_log!(
                        "Received request for struct {}: {:?}",
                        stringify!(#struct_name),
                        msg
                    );

                    #handler_code
                }

                Ok::<(), async_nats::Error>(())
            }
        });
    }
}

/// Generate the reply block that sends responses back to the client
///
/// This function generates code that checks if a message has a reply subject
/// and sends the response back to the client. This is used for request-reply
/// message handlers.
///
/// # Arguments
///
/// * `struct_name` - The name of the struct (used for logging)
///
/// # Returns
///
/// Returns a `TokenStream` containing the reply sending code.
fn generate_reply_block(struct_name: &syn::Ident) -> TokenStream {
    quote! {
        if let Some(reply) = msg.reply {
            message_flow::logger::info_log!(
                "Sending reply for struct {}: {:?}",
                stringify!(#struct_name),
                reply
            );
            let byte_response = bytes::Bytes::from(body);
            let _ = client.publish(reply, byte_response).await?;
        }
    }
}

/// Generate handler code for message handlers
///
/// This function generates code based on the handler signature:
/// - If the handler has `&self`, it is treated as a static schema handler:
///   the payload is deserialized into the struct and the method is called on `self`.
/// - If the handler has a `serde_json::Value` parameter and no `&self`, it is treated
///   as a dynamic schema handler: the payload is deserialized into `serde_json::Value`
///   and passed to an associated function on the struct.
///
/// Any other combination (both `&self` and `serde_json::Value`, or neither) will
/// result in a compile-time error via `compile_error!`.
fn generate_message_handler_conditional(
    struct_name: &syn::Ident,
    function_name: &TokenStream,
    metadata: &HandlerMetadata,
    reply_block: &TokenStream,
) -> TokenStream {
    match (metadata.has_self, metadata.has_json_value) {
        // Static schema: method on &self
        (true, false) => {
            if metadata.has_context {
                quote! {
                    match serde_json::from_slice::<#struct_name>(msg.payload.as_ref()) {
                        Ok(resolver) => {
                            let result = resolver.#function_name(&*context).await;
                            let body = match result {
                                Ok(val) => val.to_json(),
                                Err(_err) => (context.error_handler)(500, _err.to_string(), client).to_json(),
                            };
                            #reply_block
                        }
                        Err(_error) => {
                            let body = (context.error_handler)(422, _error.to_string(), client).to_json();
                            #reply_block
                        }
                    }
                }
            } else {
                quote! {
                    match serde_json::from_slice::<#struct_name>(msg.payload.as_ref()) {
                        Ok(resolver) => {
                            let result = resolver.#function_name().await;
                            let body = match result {
                                Ok(val) => val.to_json(),
                                Err(_err) => (context.error_handler)(500, _err.to_string(), client).to_json(),
                            };
                            #reply_block
                        }
                        Err(_error) => {
                            let body = (context.error_handler)(422, _error.to_string(), client).to_json();
                            #reply_block
                        }
                    }
                }
            }
        }
        // Dynamic schema: associated function taking serde_json::Value
        (false, true) => {
            if metadata.has_context {
                quote! {
                    match serde_json::from_slice::<serde_json::Value>(msg.payload.as_ref()) {
                        Ok(input_value) => {
                            let result = <#struct_name>::#function_name(&*context, input_value).await;
                            let body = match result {
                                Ok(val) => val.to_json(),
                                Err(_err) => (context.error_handler)(500, _err.to_string(), client).to_json(),
                            };
                            #reply_block
                        }
                        Err(_error) => {
                            let body = (context.error_handler)(422, _error.to_string(), client).to_json();
                            #reply_block
                        }
                    }
                }
            } else {
                quote! {
                    match serde_json::from_slice::<serde_json::Value>(msg.payload.as_ref()) {
                        Ok(input_value) => {
                            let result = <#struct_name>::#function_name(input_value).await;
                            let body = match result {
                                Ok(val) => val.to_json(),
                                Err(_err) => (context.error_handler)(500, _err.to_string(), client).to_json(),
                            };
                            #reply_block
                        }
                        Err(_error) => {
                            let body = (context.error_handler)(422, _error.to_string(), client).to_json();
                            #reply_block
                        }
                    }
                }
            }
        }
        // Invalid combinations: generate a compile error in user code
        _ => quote! {
            compile_error!(
                "Invalid handler signature: use either `&self` (static schema) \
                 or `serde_json::Value` parameter (dynamic schema), but not both"
            );
        },
    }
}

/// Generate handler code for event handlers
///
/// This mirrors `generate_message_handler_conditional` but without reply handling.
fn generate_event_handler_conditional(
    struct_name: &syn::Ident,
    function_name: &TokenStream,
    metadata: &HandlerMetadata,
) -> TokenStream {
    match (metadata.has_self, metadata.has_json_value) {
        // Static schema: method on &self
        (true, false) => {
            if metadata.has_context {
                quote! {
                    if let Ok(resolver) = serde_json::from_slice::<#struct_name>(msg.payload.as_ref()) {
                        resolver.#function_name(&*context).await;
                    }
                }
            } else {
                quote! {
                    if let Ok(resolver) = serde_json::from_slice::<#struct_name>(msg.payload.as_ref()) {
                        resolver.#function_name().await;
                    }
                }
            }
        }
        // Dynamic schema: associated function taking serde_json::Value
        (false, true) => {
            if metadata.has_context {
                quote! {
                    if let Ok(input_value) = serde_json::from_slice::<serde_json::Value>(msg.payload.as_ref()) {
                        <#struct_name>::#function_name(&*context, input_value).await;
                    }
                }
            } else {
                quote! {
                    if let Ok(input_value) = serde_json::from_slice::<serde_json::Value>(msg.payload.as_ref()) {
                        <#struct_name>::#function_name(input_value).await;
                    }
                }
            }
        }
        // Invalid combinations: generate a compile error in user code
        _ => quote! {
            compile_error!(
                "Invalid handler signature: use either `&self` (static schema) \
                 or `serde_json::Value` parameter (dynamic schema), but not both"
            );
        },
    }
}

