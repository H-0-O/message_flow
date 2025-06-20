use std::collections::HashMap;

use darling::{FromAttributes, FromMeta};
use quote::quote;
use syn::{ImplItem, ItemImpl, Type};

use crate::{error::GeneratorResult, MsgFlowArgs};

const REGISTER_TRAIT_PATH: &str = "message_flow::Register";
const HANDLER_TRAIT_PATH: &str = "message_flow::Handler";

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
    fn from_attribute(attr: &syn::Attribute) -> Result<Self, darling::Error> {
        if attr.path().is_ident("message") {
            let parsed = MessageAttribute::from_attributes(&[attr.clone()])?;
            Ok(Attributes::Message(parsed))
        } else if attr.path().is_ident("event") {
            let parsed = EventAttribute::from_attributes(&[attr.clone()])?;
            Ok(Attributes::Event(parsed))
        } else {
            Err(darling::Error::custom("Unknown attribute"))
        }
    }
}

pub fn generate(__input: ItemImpl, args: MsgFlowArgs) -> GeneratorResult {
    let struct_name = match *__input.self_ty {
        Type::Path(ref type_path) => {
            if let Some(ident) = type_path.path.get_ident() {
                ident.clone()
            } else {
                panic!("Expected an identifier for the struct name");
            }
        }
        _ => panic!("Unsupported type for self_ty"),
    };
    let expanded_register_trait = generate_impl_register_trait(&struct_name, &args);
    let expanded_handler_trait = generate_impl_handler_trait(&__input, &struct_name, &args)?;

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

            #expanded_handler_trait

            impl #struct_name {
                #(#methods)*
            }
        };

    }
    .into())
}

fn generate_impl_register_trait(
    struct_name: &syn::Ident,
    args: &MsgFlowArgs,
) -> proc_macro2::TokenStream {
    let register_trait_path = syn::Path::from_string(REGISTER_TRAIT_PATH).unwrap();

    let pattern = format!("{}.>", &args.pattern);
    let expanded = quote! {

        #[allow(unused_imports)]
        use #register_trait_path;

        #[automatically_derived]
        #[message_flow::async_trait]
        impl #register_trait_path for #struct_name {
            async fn register(client: std::sync::Arc<message_flow::Client>) -> message_flow::Result<()> {
                let mut subscribe = client.subscribe(#pattern).await?;

                tokio::spawn({
                    let client = client.clone();

                    async move {
                        while let Some(request) = subscribe.next().await {
                            println!("THE REQUEST {:?} " , request);
                            let __result = #struct_name::router(
                                &request.subject.to_string() , request.payload.as_ref()
                            ).await;
                            println!("OK THE __RESULT {:?} " , __result);
                            if let Err(err) = __result {
                                return Err(err);
                            };

                            if let Some(reply) = request.reply {
                                let _ = client.
                                publish(reply , __result.unwrap().to_json().into())
                                .await?;
                            }
                        }
                        Ok::<(), async_nats::Error>(())
                    }
                });
                Ok(())
            }
        }
    };

    expanded
}

fn generate_impl_handler_trait(
    __input: &ItemImpl,
    struct_name: &syn::Ident,
    args: &MsgFlowArgs,
) -> GeneratorResult {
    let handler_trait_path = syn::Path::from_string(HANDLER_TRAIT_PATH).unwrap();

    let mut messages: HashMap<String, Vec<proc_macro2::TokenStream>> = HashMap::new();
    let mut events: HashMap<String, Vec<proc_macro2::TokenStream>> = HashMap::new();

    let _ = __input
        .items
        .iter()
        .map(|item| {
            if let ImplItem::Fn(func) = item {
                let func_name = &func.sig.ident;
                for attr in &func.attrs {
                    match Attributes::from_attribute(&attr) {
                        //TODO here must do some validations
                        Ok(Attributes::Message(message)) => {
                            if !messages.contains_key(&message.pattern) {
                                messages.insert(message.pattern.clone(), vec![]);
                            }
                            // the resolver is InComeMessage struct that user defined that
                            messages.get_mut(&message.pattern).unwrap().push(quote! {
                                    ::std::boxed::Box::new(resolver.#func_name().await?)
                            });
                        }
                        Ok(Attributes::Event(event)) => {
                            if !events.contains_key(&event.pattern) {
                                events.insert(event.pattern.clone(), vec![]);
                            }
                            // the resolver is InComeMessage struct that user defined that
                            events.get_mut(&event.pattern).unwrap().push(quote! {
                                resolver.#func_name().await
                            });
                        }
                        Err(err) => {
                            panic!("Error parsing attribute: {}", err);
                        }
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    // println!("THE EVENTS : {:?} ", events);
    let messages_token_stream: Vec<proc_macro2::TokenStream> = messages
        .iter()
        .map(|(pattern, fns)| {
            let temp_first = &fns[0];
            let _pattern = format!("{}.{}", args.pattern, pattern);
            quote! {
                #_pattern => #temp_first
            }
        })
        .collect();

    let events_token_stream: Vec<proc_macro2::TokenStream> = events
        .iter()
        .map(|(pattern, fns)| {
            let temp_first = &fns[0];
            let _pattern = format!("{}.{}", args.pattern, pattern);
            quote! {
                #_pattern => #temp_first
            }
        })
        .collect();

    // println!("THE PATTERN {:?} " , messages_token_stream.to_vec());
    //TODO implement the events
    let expanded = quote! {

        #[allow(unused_imports)]
        use #handler_trait_path;

        #[automatically_derived]
        #[message_flow::async_trait]
        impl #handler_trait_path for #struct_name {
            async fn router(subject: &String, payload: &[u8]) -> message_flow::Result<::std::boxed::Box<dyn message_flow::Message>> {
                let s = std::str::from_utf8(payload).unwrap(); // Safe if valid UTF-8
                // println!("THE SS  {:?} " , s);
                let resolver = message_flow::InComeMessage::<Self>::new(payload);
                // let resolver = serde_json::from_slice::<Self>(payload).unwrap();
                println!("IN HANDLE and message {:?} ", subject.as_str());
                let func: ::std::boxed::Box<dyn message_flow::Message> = match subject.as_str() {
                    #(#messages_token_stream),*,
                    _ => return Err(async_nats::Error::from(format!("Can not find subscriber of {}" , subject.as_str()))),
                };

                Ok(func)
            }
        }

    };

    Ok(expanded)
}
