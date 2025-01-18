use darling::FromMeta;
use quote::{quote, ToTokens};
use syn::{self, ItemStruct};

use crate::error::GeneratorResult;

const MESSAGE_TRAIT_PATH: &str = "message_flow::Message";

pub fn generate(__input: ItemStruct) -> GeneratorResult {
    let struct_name = &__input.ident;
    let message_trait_path = syn::Path::from_string(MESSAGE_TRAIT_PATH).unwrap();
    let expanded = quote! {
        #[automatically_derived]
        impl #message_trait_path for #struct_name {}
    };

    Ok(expanded.into())
}
