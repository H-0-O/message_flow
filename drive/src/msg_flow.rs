use darling::{ast::NestedMeta, util, FromMeta};
use quote::{quote, ToTokens};
use syn::{ImplItem, ItemImpl, Type};

use crate::{
    error::{Error, GeneratorResult},
    MsgFlowArgs,
};

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
    let expanded_handler_trait = generate_impl_handler_trait(&__input, &struct_name)?;

    let items = &__input.items;

    let methods: Vec<proc_macro2::TokenStream> = items
        .iter()
        .map(|f| {
            if let ImplItem::Fn(func) = f {
                return quote! { #func };
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

    let pattern = &args.pattern;
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

fn generate_impl_handler_trait(__input: &ItemImpl, struct_name: &syn::Ident) -> GeneratorResult {
    let handler_trait_path = syn::Path::from_string(HANDLER_TRAIT_PATH).unwrap();

    #[derive(Debug, FromMeta)]
    struct FunctionArgs {
        pattern: String,
    }

    let patterns: Vec<proc_macro2::TokenStream> = {
        __input
            .items
            .iter()
            .map(|item| {
                if let ImplItem::Fn(func) = item {
                    panic!("IN THE FIRST");
                    let attrs = &func.attrs;
                    let function_args = {
                        let tokens = attrs.iter().map(|attr| attr.to_token_stream());
                        let token_stream = proc_macro2::TokenStream::from_iter(tokens);
                        let args = match NestedMeta::parse_meta_list(token_stream) {
                            Ok(v) => v,
                            Err(error) => return Error::from(error).write_errors(),
                        };
                        match FunctionArgs::from_list(&args) {
                            Ok(v) => v,
                            Err(error) => return Error::from(error).write_errors(),
                        }
                    };
                    let ee = func.sig.to_token_stream();
                    return quote! {#ee};
                }
                quote! {#item}
            })
            .collect()
    };
    // panic!("THE PATTERNS {:?} ", patterns);
    let expanded = quote! {

        #[allow(unused_imports)]
        use #handler_trait_path;

        #[automatically_derived]
        #[message_flow::async_trait]
        impl #handler_trait_path for #struct_name {
            async fn router(subject: &String, payload: &[u8]) -> message_flow::Result<::std::boxed::Box<dyn message_flow::Message>> {

                let resolver = serde_json::from_slice::<Self>(payload).unwrap();
                println!("IN HANDLE and message {:?} ", subject);
                let func: ::std::boxed::Box<dyn message_flow::Message> = match subject.as_str() {
                    _ => return Err(async_nats::Error::from("Pattern Not found")),
                };

                Ok(func)
            }
        }

    };

    Ok(expanded)
}
