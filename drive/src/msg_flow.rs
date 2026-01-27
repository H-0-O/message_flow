use quote::quote;
use syn::{ImplItem, ItemImpl, Type};

use crate::{
    error::GeneratorResult,
    register::generate_register_impl,
    MsgFlowArgs,
};

/// Extract the struct name from an impl block
///
/// Returns an error if the impl block is not for a simple struct type.
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
/// This generates:
/// 1. The Register trait implementation
/// 2. The original impl block with all methods preserved
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
