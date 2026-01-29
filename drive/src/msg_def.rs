//! Code generation for the `MsgDef` derive macro
//!
//! This module generates the `Message` trait implementation for structs that
//! derive `MsgDef`.

use quote::quote;
use syn::{parse_str, ItemStruct};

use crate::{constants::MESSAGE_TRAIT_PATH, error::GeneratorResult};

/// Generate the `Message` trait implementation for a struct
///
/// This function takes a struct definition and generates code that implements
/// the `message_flow::Message` trait for it.
///
/// # Arguments
///
/// * `input` - The struct definition parsed from the `#[derive(MsgDef)]` attribute
///
/// # Returns
///
/// Returns a `TokenStream` containing the generated trait implementation, or
/// an error if code generation fails.
///
/// # Generated Code
///
/// Generates code like:
/// ```rust,ignore
/// #[automatically_derived]
/// impl message_flow::Message for MyStruct {
///     const IS_UNIT: bool = true; // or false
/// }
/// ```
pub fn generate(input: ItemStruct) -> GeneratorResult {
    let struct_name = &input.ident;
    let message_trait_path: syn::Path = parse_str(MESSAGE_TRAIT_PATH)?;

    // Detect if struct is unit-like (no fields)
    let is_unit = matches!(input.fields, syn::Fields::Unit);

    let expanded = quote! {
        #[automatically_derived]
        impl #message_trait_path for #struct_name {
            const IS_UNIT: bool = #is_unit;
        }
    };

    Ok(expanded.into())
}
