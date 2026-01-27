use quote::quote;
use syn::{parse_str, ItemStruct};

use crate::{
    constants::MESSAGE_TRAIT_PATH,
    error::GeneratorResult,
};

pub fn generate(input: ItemStruct) -> GeneratorResult {
    let struct_name = &input.ident;
    let message_trait_path: syn::Path = parse_str(MESSAGE_TRAIT_PATH)?;
    let expanded = quote! {
        #[automatically_derived]
        impl #message_trait_path for #struct_name {}
    };

    Ok(expanded.into())
}
