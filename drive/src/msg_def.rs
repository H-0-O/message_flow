use quote::{quote, ToTokens};
use syn::{self, ItemStruct};

use crate::error::GeneratorResult;

pub fn generate(__input: ItemStruct) -> GeneratorResult {
    let the_struct = __input.to_token_stream();
    let impl_message_trait = quote! {};
    Ok(the_struct.into())
}
