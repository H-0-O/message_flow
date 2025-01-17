use proc_macro::TokenStream;
use quote::quote;
use thiserror::Error;

pub type GeneratorResult = Result<TokenStream, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Pattern Can not be empty")]
    PatternIsEmpty,

    #[error("Darling error: {0}")]
    DarlingError(#[from] darling::Error),

    #[error("Syn error: {0}")]
    SynError(#[from] syn::Error),
}

impl Error {
    pub fn write_errors(self) -> TokenStream {
        let message = self.to_string();
        quote! {
            compile_error!(#message)
        }
        .into()
    }
}
