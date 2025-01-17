use darling::{ast::NestedMeta, FromMeta};
use error::Error;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod error;
mod msg_def;

//---------------------------------- MsgDef
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn MsgDef(attr: TokenStream, item: TokenStream) -> TokenStream {
    let __input = parse_macro_input!(item as syn::ItemStruct);

    let generated_tokens = msg_def::generate(__input);
    match generated_tokens {
        Ok(v) => v,
        Err(e) => e.write_errors(),
    }
}

//------------------------------- msg_flow
#[derive(Debug, FromMeta)]
struct MsgFlowArgs {
    prefix: Option<String>,
}
#[proc_macro_attribute]
pub fn msg_flow(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemImpl);

    quote! {#input}.into()
}

//---------------------------------- msg_pattern
#[derive(Debug, FromMeta)]
struct Args {
    pattern: String,
}

#[proc_macro_attribute]
pub fn msg_pattern(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    let f_name = input.sig.ident.to_string();

    let args = {
        let attr_args = match NestedMeta::parse_meta_list(attr.into()) {
            Ok(v) => v,
            Err(e) => return Error::from(e).write_errors(),
        };

        match Args::from_list(&attr_args) {
            Ok(v) => v,
            Err(e) => return Error::from(e).write_errors(),
        }
    };

    quote! {#input}.into()
}

#[proc_macro_attribute]
pub fn event_pattern(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);

    quote! {#input}.into()
}
