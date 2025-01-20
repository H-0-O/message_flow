use darling::FromMeta;
use error::Error;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod error;
mod msg_def;
mod msg_flow;

macro_rules! parse_nested_meta {
    ($ty:ty, $args:expr) => {{
        let meta = match darling::ast::NestedMeta::parse_meta_list($args.into()) {
            Ok(v) => v,
            Err(err) => {
                return Error::from(err).write_errors().into();
            }
        };

        match <$ty>::from_list(&meta) {
            Ok(object_args) => object_args,
            Err(err) => return Error::from(err).write_errors().into(),
        }
    }};
}

//---------------------------------- MsgDef
#[proc_macro_derive(MsgDef)]
pub fn msg_def(item: TokenStream) -> TokenStream {
    let __input = parse_macro_input!(item as syn::ItemStruct);

    let generated_tokens = msg_def::generate(__input);
    match generated_tokens {
        Ok(v) => v,
        Err(e) => e.write_errors(),
    }
    .into()
}

//------------------------------- msg_flow
#[derive(Debug, FromMeta)]
pub(crate) struct MsgFlowArgs {
    pattern: String,
}
#[proc_macro_attribute]
pub fn msg_flow(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemImpl);
    let args = parse_nested_meta!(MsgFlowArgs, attr);
    let generated_tokens = msg_flow::generate(input, args);

    match generated_tokens {
        Ok(v) => v,
        Err(e) => e.write_errors(),
    }
    .into()
}

//---------------------------------- msg_pattern
#[derive(Debug, FromMeta)]
struct MsgPatternArgs {
    pattern: String,
}

// #[proc_macro_attribute]
// pub fn msg_pattern(attr: TokenStream, item: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(item as syn::ItemFn);
//     // let args = parse_nested_meta!(MsgPatternArgs, attr);

//     quote! {#input}.into()
// }

#[proc_macro_attribute]
pub fn event_pattern(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);

    quote! {#input}.into()
}
