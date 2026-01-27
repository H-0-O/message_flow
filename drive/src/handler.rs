//! Handler function analysis and code generation
//!
//! This module provides utilities for analyzing handler function signatures and
//! generating code to invoke them with the appropriate arguments.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, FnArg, Token, Type};

use crate::constants::{CONTEXT_FULL_PATH, CONTEXT_TYPE_NAME};

/// Check if a function has a Context parameter
///
/// This function examines the function signature to determine if it takes
/// a `Context` or `message_flow::Context` parameter (by reference).
pub fn has_context_parameter(inputs: &Punctuated<FnArg, Token![,]>) -> bool {
    inputs.iter().any(|input| {
        match input {
            FnArg::Receiver(_) => false, // `self` parameter
            FnArg::Typed(typed) => {
                let inner_ty = match typed.ty.as_ref() {
                    Type::Reference(ref_ref) => ref_ref.elem.as_ref(),
                    other => other,
                };

                if let Type::Path(type_path) = inner_ty {
                    let type_string = type_path
                        .path
                        .segments
                        .iter()
                        .map(|seg| seg.ident.to_string())
                        .collect::<Vec<_>>()
                        .join("::");
                    type_string == CONTEXT_TYPE_NAME || type_string == CONTEXT_FULL_PATH
                } else {
                    false
                }
            }
        }
    })
}

/// Generate the input arguments for invoking a handler function
///
/// If the handler has a Context parameter, this generates `&*context`,
/// otherwise it generates an empty token stream.
pub fn generate_handler_inputs(has_ctx: bool) -> TokenStream {
    if has_ctx {
        quote! { &*context }
    } else {
        quote! {}
    }
}

