use proc_macro2::TokenStream;
use quote::quote;
use thiserror::Error;

pub type GeneratorResult = Result<TokenStream, syn::Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Darling error: {0}")]
    DarlingError(#[from] darling::Error),

    #[error("Syn error: {0}")]
    SynError(#[from] syn::Error),
}

#[derive(Debug, Error)]
pub enum AttributeParseError {

    #[error("Message Pattern Can not be empty")]
    MessagePatternIsEmpty,

    #[error("Event Pattern Can not be empty")]
    EventPatternIsEmpty,

    #[error("Unknown attribute")]
    UnknownAttribute,
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
