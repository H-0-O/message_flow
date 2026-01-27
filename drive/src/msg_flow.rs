//! Code generation for the `#[msg_flow]` attribute macro
//!
//! This module generates the `Register` trait implementation and preserves the
//! original impl block methods when processing `#[msg_flow]` attributes.

use quote::quote;
use syn::{ImplItem, ItemImpl, Type};

use crate::{
    error::GeneratorResult,
    register::generate_register_impl,
    MsgFlowArgs,
};

/// Extract the struct name from an impl block
///
/// This function extracts the type name from an `impl` block. It only supports
/// simple struct types (not generic types or complex paths).
///
/// # Arguments
///
/// * `impl_block` - The impl block to extract the struct name from
///
/// # Returns
///
/// Returns the identifier of the struct type, or an error if the impl block
/// is not for a simple struct type.
///
/// # Errors
///
/// Returns an error if:
/// - The impl block is for a generic type
/// - The impl block is for a complex type path
/// - The type cannot be identified
fn extract_struct_name(impl_block: &ItemImpl) -> Result<syn::Ident, syn::Error> {
    match *impl_block.self_ty {
        Type::Path(ref type_path) => {
            if let Some(ident) = type_path.path.get_ident() {
                Ok(ident.clone())
            } else {
                Err(syn::Error::new_spanned(
                    type_path,
                    "Expected an identifier for the struct name",
                ))
            }
        }
        _ => Err(syn::Error::new_spanned(
            impl_block,
            "Unsupported type for self_ty. Expected a simple struct type.",
        )),
    }
}

/// Generate the complete code for a `#[msg_flow]` impl block
///
/// This function generates:
/// 1. The `Register` trait implementation that registers all handlers with the broker
/// 2. The original impl block with all methods preserved
///
/// The generated code is wrapped in a `const _: ()` block to ensure proper scoping
/// and to allow the generated code to coexist with the original impl block.
///
/// # Arguments
///
/// * `impl_block` - The impl block annotated with `#[msg_flow]`
/// * `args` - The parsed arguments from the `#[msg_flow]` attribute
///
/// # Returns
///
/// Returns a `TokenStream` containing:
/// - The generated `Register` trait implementation
/// - The original impl block with all methods preserved
///
/// # Errors
///
/// Returns an error if:
/// - The struct name cannot be extracted
/// - The register implementation cannot be generated
///
/// # Example Generated Code
///
/// ```rust,ignore
/// const _: () = {
///     impl Register for MyStruct { ... }
///     impl MyStruct {
///         async fn handler1(&self) { ... }
///         async fn handler2(&self) { ... }
///     }
/// };
/// ```
pub fn generate(impl_block: ItemImpl, args: MsgFlowArgs) -> GeneratorResult {
    let struct_name = extract_struct_name(&impl_block)?;

    let register_impl = generate_register_impl(&struct_name, &impl_block, &args)?;

    // Preserve all original methods from the impl block
    let methods: Vec<proc_macro2::TokenStream> = impl_block
        .items
        .iter()
        .map(|item| {
            if let ImplItem::Fn(func) = item {
                let func_sig = &func.sig;
                let func_body = &func.block;
                quote! {
                    #func_sig {
                        #func_body
                    }
                }
            } else {
                quote! {}
            }
        })
        .collect();

    Ok(quote! {
        const _: () = {
            #register_impl

            impl #struct_name {
                #(#methods)*
            }
        };
    }
    .into())
}
