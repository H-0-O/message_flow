use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_str, ImplItem, ItemImpl};

use crate::{
    attribute::AttributeType,
    constants::REGISTER_TRAIT_PATH,
    error::GeneratorResult,
    handler::{generate_handler_inputs, has_context_parameter},
    MsgFlowArgs,
};

/// Represents the type of handler (Message or Event)
#[derive(Debug, Clone, Copy)]
pub enum HandlerType {
    Message,
    Event,
}

/// Metadata about a handler function
#[derive(Debug)]
pub struct HandlerMetadata {
    pub handler_type: HandlerType,
    pub function_name: proc_macro2::TokenStream,
    pub has_context: bool,
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
                    },
                );
            }
        }
    }

    Ok(handlers)
}

/// Generate the implementation of the Register trait
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
fn generate_subscription_code(
    struct_name: &syn::Ident,
    pattern: &str,
    metadata: &HandlerMetadata,
    base_pattern: &str,
) -> TokenStream {
    let topic_pattern = format!("{}{}", base_pattern, pattern);
    let function_name = &metadata.function_name;
    let handler_inputs = generate_handler_inputs(metadata.has_context);

    let reply_block = generate_reply_block(struct_name);
    let handler_code = match metadata.handler_type {
        HandlerType::Message => generate_message_handler(function_name, &handler_inputs, &reply_block),
        HandlerType::Event => generate_event_handler(function_name, &handler_inputs),
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

                    match serde_json::from_slice::<#struct_name>(msg.payload.as_ref()) {
                        Ok(resolver) => {
                            #handler_code
                        }
                        Err(_error) => {
                            let response = (context.error_handler)(422, _error.to_string(), client);
                            #reply_block
                        }
                    }
                }

                Ok::<(), async_nats::Error>(())
            }
        });
    }
}

/// Generate the reply block that sends responses back to the client
fn generate_reply_block(struct_name: &syn::Ident) -> TokenStream {
    quote! {
        if let Some(reply) = msg.reply {
            message_flow::logger::info_log!(
                "Sending reply for struct {}: {:?}",
                stringify!(#struct_name),
                reply
            );
            let byte_response = bytes::Bytes::from(response.to_json());
            let _ = client.publish(reply, byte_response).await?;
        }
    }
}

/// Generate handler code for message handlers (request-reply pattern)
fn generate_message_handler(
    function_name: &TokenStream,
    inputs: &TokenStream,
    reply_block: &TokenStream,
) -> TokenStream {
    quote! {
        let result = resolver.#function_name(#inputs).await;

        let response: ::std::boxed::Box<dyn message_flow::Message> = match result {
            Ok(val) => ::std::boxed::Box::new(val),
            Err(_err) => (context.error_handler)(500, _err.to_string(), client),
        };

        #reply_block
    }
}

/// Generate handler code for event handlers (fire-and-forget pattern)
fn generate_event_handler(function_name: &TokenStream, inputs: &TokenStream) -> TokenStream {
    quote! {
        resolver.#function_name(#inputs).await;
    }
}

