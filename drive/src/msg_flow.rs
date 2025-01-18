use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ImplItem, ItemImpl, Type};

use crate::{error::GeneratorResult, MsgFlowArgs};

const REGISTER_TRAIT_PATH: &str = "message_flow::Register";
const HANDLER_TRAIT_PATH: &str = "message_flow::Handler";

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
    let expanded_handler_trait = generate_impl_handler_trait(&__input, &struct_name);
    Ok(quote! {
        #expanded_register_trait

        #expanded_handler_trait
    }
    .into())
}

fn generate_impl_register_trait(
    struct_name: &syn::Ident,
    args: &MsgFlowArgs,
) -> proc_macro2::TokenStream {
    let register_trait_path = syn::Path::from_string(REGISTER_TRAIT_PATH).unwrap();
    let pattern = &args.pattern;
    let expanded = quote! {
        #[automatically_derived]
        #[message_flow::async_trait]
        impl #register_trait_path for #struct_name {
            async fn register(client: std::sync::Arc<message_flow::Client>) -> message_flow::Result<()> {
                let subscribe = client.subscribe(#pattern).await?;

                tokio::spawn({
                    let client = client.clone();

                    async move {
                        while let Some(request) = subscribe.next().await {
                            let __result = #struct_name::router(
                                &request.subject.to_string() , request.payload.as_ref()
                            ).await;

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
) -> proc_macro2::TokenStream {
    let handler_trait_path = syn::Path::from_string(HANDLER_TRAIT_PATH).unwrap();

    let methods = &__input.items;

    let ew: Vec<_> = methods
        .into_iter()
        .map(|f| {
            if let ImplItem::Fn(func) = f {
                let vis = &func.vis;
                let body = &func.block;
                let fn_name = &func.sig;
                return quote! {};
            };

            quote! {}
        })
        .collect();
    let expanded = quote! {

        impl #handler_trait_path for #struct_name {
            async fn router(subject: &String, payload: &[u8]) -> message_flow::Result<::std::boxed::Box<dyn message_flow::Message>> {

                let resolver = serde_json::from_slice::<Self>(payload).unwrap();
                println!("IN HANDLE and message {:?} ", subject);
                let func: ::std::boxed::Box<dyn message_flow::Message> = match subject.as_str() {
                    "service_A.greeting" => ::std::boxed::Box::new(resolver.greeting().await?),
                    _ => return Err(async_nats::Error::from("Pattern Not found")),
                };

                Ok(func)
            }
        }

    };

    expanded
}
