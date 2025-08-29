use std::collections::HashMap;

use darling::{FromAttributes, FromMeta};
use quote::quote;
use syn::{ImplItem, ItemImpl, Type};

use crate::{
    error::{AttributeParseError, GeneratorResult},
    MsgFlowArgs,
};

const REGISTER_TRAIT_PATH: &str = "message_flow::Register";

#[derive(Debug)]
enum Attributes {
    Message(MessageAttribute),
    Event(EventAttribute),
}
#[derive(Debug, FromAttributes)]
#[darling(attributes(message))]
struct MessageAttribute {
    pattern: String,
}

#[derive(Debug, FromAttributes)]
#[darling(attributes(event))]
struct EventAttribute {
    pattern: String,
}

impl Attributes {
    fn from_attribute(attr: &syn::Attribute) -> Result<Self, syn::Error> {
        if attr.path().is_ident("message") {
            let parsed = MessageAttribute::from_attributes(&[attr.clone()])?;
            if parsed.pattern.is_empty() {
                return Err(syn::Error::new_spanned(
                    attr,
                    AttributeParseError::MessagePatternIsEmpty.to_string(),
                ));
            }
            Ok(Attributes::Message(parsed))
        } else if attr.path().is_ident("event") {
            let parsed = EventAttribute::from_attributes(&[attr.clone()])?;
            if parsed.pattern.is_empty() {
                return Err(syn::Error::new_spanned(
                    attr,
                    AttributeParseError::EventPatternIsEmpty.to_string(),
                ));
            }
            Ok(Attributes::Event(parsed))
        } else {
            return Err(syn::Error::new_spanned(
                attr,
                AttributeParseError::UnknownAttribute.to_string(),
            ));
        }
    }
}

pub fn generate(__input: ItemImpl, args: MsgFlowArgs) -> GeneratorResult {
    let struct_name = match *__input.self_ty {
        Type::Path(ref type_path) => {
            if let Some(ident) = type_path.path.get_ident() {
                ident.clone()
            } else {
                return Err(syn::Error::new_spanned(
                    type_path,
                    "Expected an identifier for the struct name",
                ));
            }
        }
        _ => {
            return Err(syn::Error::new_spanned(
                __input,
                "Unsupported type for self_ty",
            ))
        }
    };
    let expanded_register_trait = match generate_impl_register_trait(&__input, &struct_name, &args)
    {
        Ok(token) => token,
        Err(e) => return Err(e),
    };

    // let expanded_handler_trait = generate_impl_handler_trait(&__input, &struct_name, &args)?;

    let items = &__input.items;

    let methods: Vec<proc_macro2::TokenStream> = items
        .iter()
        .map(|item| {
            if let ImplItem::Fn(func) = item {
                let func_sig = &func.sig;
                let func_body = &func.block;
                return quote! {
                   #func_sig {
                       #func_body
                   }
                };
            };

            quote! {}
        })
        .collect();

    Ok(quote! {

        const _: () = {

            #expanded_register_trait



            impl #struct_name {
                #(#methods)*
            }
        };

    }
    .into())
}

#[derive(Debug)]
enum MacroType {
    Message,
    Event,
}
#[derive(Debug)]
struct AttrMacro {
    macro_type: MacroType,
    invokable_function: proc_macro2::TokenStream,
}

fn generate_impl_register_trait(
    __input: &ItemImpl,
    struct_name: &syn::Ident,
    args: &MsgFlowArgs,
) -> GeneratorResult {
    let register_trait_path = syn::Path::from_string(REGISTER_TRAIT_PATH).unwrap();

    let base_pattern = match &args.pattern {
        Some(pattern) => format!("{}.", pattern),
        None => "".into(),
    };
    // the Key is pattern defined by user
    let mut macros: HashMap<String, AttrMacro> = HashMap::new();

    for item in &__input.items {
        if let ImplItem::Fn(func) = item {
            let func_name = &func.sig.ident;
            for attr in &func.attrs {
                match Attributes::from_attribute(&attr) {
                    //TODO here must do some validations
                    Ok(Attributes::Message(message)) => {
                        if !macros.contains_key(&message.pattern) {
                            macros.insert(
                                message.pattern.clone(),
                                AttrMacro {
                                    macro_type: MacroType::Message,
                                    invokable_function: quote! {},
                                },
                            );
                        }
                        macros.get_mut(&message.pattern).unwrap().invokable_function = {
                            quote! {
                                    #func_name
                            }
                        };
                    }
                    Ok(Attributes::Event(event)) => {
                        if !macros.contains_key(&event.pattern) {
                            macros.insert(
                                event.pattern.clone(),
                                AttrMacro {
                                    macro_type: MacroType::Event,
                                    invokable_function: quote! {},
                                },
                            );
                        }

                        macros.get_mut(&event.pattern).unwrap().invokable_function = {
                            quote! {
                                    #func_name
                            }
                        };
                    }
                    Err(err) => return Err(err),
                }
            }
        }
    }

    let messages_token_stream: Vec<proc_macro2::TokenStream> = macros
        .iter()
        .map(|(pattern, macro_attr)| {
            let function_to_invoke = &macro_attr.invokable_function;
            let topic_pattern = format!("{}{}", base_pattern, pattern);

            let reply_block = quote! {
                if let Some(reply) = msg.reply {
                    message_flow::logger::info_log!(
                        "Sending reply for struct {}: {:?}",
                        stringify!(#struct_name),
                        reply
                    );
                    let byte_response = bytes::Bytes::from(response.to_json());
                    let _ = client.publish(reply, byte_response).await?;
                }
            };

            let handler = match macro_attr.macro_type {
                MacroType::Message => {
                    quote! {
                        let result = resolver.#function_to_invoke().await;

                        let response: ::std::boxed::Box<dyn message_flow::Message> = match result {
                            Ok(val) => ::std::boxed::Box::new(val),
                            Err(_err) => (context.error_handler)(500 , _err.to_string() , client),
                        };

                        #reply_block
                    }
                }
                MacroType::Event => {
                    quote! {
                        resolver.#function_to_invoke().await
                    }
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

                            match serde_json::from_slice::<#struct_name>(msg.payload.as_ref()) {
                                Ok(resolver) => {
                                    #handler
                                }
                                Err(_error) => {
                                    let response = (context.error_handler)(422 , _error.to_string() , client);
                                    #reply_block
                                }
                            }
                        }

                        Ok::<(), async_nats::Error>(())
                    }
                });
            }
        })
        .collect();

    Ok(quote! {
        #[automatically_derived]
        #[message_flow::async_trait]
        impl #register_trait_path for #struct_name {
            async fn register(context: std::sync::Arc<message_flow::Context>) -> message_flow::Result<()> {
                #(#messages_token_stream)*
                Ok(())
            }
        }
    })
}
