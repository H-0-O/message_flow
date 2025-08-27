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
                panic!("Expected an identifier for the struct name");
            }
        }
        _ => panic!("Unsupported type for self_ty"),
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
    let mut messages: HashMap<String, Vec<proc_macro2::TokenStream>> = HashMap::new();
    let mut events: HashMap<String, Vec<proc_macro2::TokenStream>> = HashMap::new();

    for item in &__input.items {
        if let ImplItem::Fn(func) = item {
            let func_name = &func.sig.ident;
            for attr in &func.attrs {
                match Attributes::from_attribute(&attr) {
                    //TODO here must do some validations
                    Ok(Attributes::Message(message)) => {
                        if !messages.contains_key(&message.pattern) {
                            messages.insert(message.pattern.clone(), vec![]);
                        }
                        messages.get_mut(&message.pattern).unwrap().push(quote! {
                                #func_name
                        });
                    }
                    Ok(Attributes::Event(event)) => {
                        if !events.contains_key(&event.pattern) {
                            events.insert(event.pattern.clone(), vec![]);
                        }
                        // the resolver is InComeMessage struct that developer defined that and we inject it later
                        events.get_mut(&event.pattern).unwrap().push(quote! {
                            #func_name
                        });
                    }
                    Err(err) => return Err(err),
                }
            }
        }
    }

    // if let Err(err) = populate_attribute_result {
    //     return err.write_errors();
    // }
    // let expanded = quote! {

    //     #[allow(unused_imports)]
    //     use #register_trait_path;

    //     #[automatically_derived]
    //     #[message_flow::async_trait]
    //     impl #register_trait_path for #struct_name {
    //         async fn register(client: std::sync::Arc<message_flow::Client>) -> message_flow::Result<()> {
    //             let pattern_list = [""];

    //             let mut subscribe = client.subscribe(#pattern).await?;

    //             message_flow::logger::info_log!("Subscribed to {} for struct {}", #pattern, stringify!(#struct_name));

    //             tokio::spawn({
    //                 let client = client.clone();

    //                 message_flow::logger::info_log!("Spawned task for struct {}", stringify!(#struct_name));

    //                 async move {
    //                     while let Some(request) = subscribe.next().await {
    //                         message_flow::logger::info_log!("Received request for struct {}: {:?}", stringify!(#struct_name), request);
    //                         let __result = #struct_name::router(
    //                             &request.subject.to_string() , request.payload.as_ref()
    //                         ).await;
    //                         message_flow::logger::info_log!("Result for struct {}: {:?}", stringify!(#struct_name), __result);

    //                         if let Err(err) = __result {
    //                             return Err(err);
    //                         };

    //                         if let Some(reply) = request.reply {
    //                             message_flow::logger::info_log!("Sending reply for struct {}: {:?}", stringify!(#struct_name), reply);
    //                             let _ = client.
    //                             publish(reply , __result.unwrap().to_json().into())
    //                             .await?;
    //                         }
    //                     }
    //                     Ok::<(), async_nats::Error>(())
    //                 }
    //             });
    //             Ok(())
    //         }
    //     }
    // };

    let messages_token_stream: Vec<proc_macro2::TokenStream> = messages
        .iter()
        .map(|(pattern, fns)| {
            let function_to_invoke = &fns[0];
            let _pattern = format!("{}{}", base_pattern, pattern);
            quote! {
                    tokio::spawn({
                        let client = client.clone();
                        async move {
                            let mut subscribe = client.subscribe(#_pattern).await?;

                            message_flow::logger::info_log!("Spawned task for struct {}", stringify!(#struct_name));

                            while let Some(msg) = subscribe.next().await {

                                message_flow::logger::info_log!("Received request for struct {}: {:?}", stringify!(#struct_name), msg);

                                let resolver = message_flow::InComeMessage::<User>::new(msg.payload.as_ref());

                                //TODO remove the ? operand to pass global error handler by defined developer or system 
                                let result: ::std::boxed::Box<dyn message_flow::Message> = ::std::boxed::Box::new(resolver.#function_to_invoke().await?);

                                if let Some(reply) = msg.reply {
                                    message_flow::logger::info_log!("Sending reply for struct {}: {:?}", stringify!(#struct_name), reply);
                                    let response = bytes::Bytes::from(result.to_json());
                                    println!("THE REPLAY {:?} " , reply);
                                    let _ = client.
                                    publish(reply , response).await?;
                                }
                            }
                            Ok::<(), async_nats::Error>(())
                        }
                });
                // #_pattern => #temp_first
            }
        })
        .collect();

    Ok(quote! {
        #[automatically_derived]
        #[message_flow::async_trait]
        impl #register_trait_path for #struct_name {
            async fn register(client: std::sync::Arc<message_flow::Client>) -> message_flow::Result<()> {
                #(#messages_token_stream)*
                Ok(())
            }
        }
    })
    // expanded
}
