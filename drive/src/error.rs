//! Error types for procedural macro code generation
//!
//! This module defines error types used throughout the macro code generation process.

use proc_macro2::TokenStream;
use quote::quote;
use thiserror::Error;

/// Result type alias for code generation functions
///
/// This is a convenience alias for `Result<TokenStream, syn::Error>` used by
/// all code generation functions in the procedural macros.
pub type GeneratorResult = Result<TokenStream, syn::Error>;

/// Main error type for procedural macro operations
///
/// This enum represents errors that can occur during macro expansion, including
/// parsing errors from `darling` and `syn`.
#[derive(Error, Debug)]
pub enum Error {
    /// Error from the `darling` attribute parsing library
    #[error("Darling error: {0}")]
    DarlingError(#[from] darling::Error),

    /// Error from the `syn` parsing library
    #[error("Syn error: {0}")]
    SynError(#[from] syn::Error),
}

/// Errors specific to attribute parsing
///
/// This enum represents errors that occur when parsing attributes like `#[message]`
/// and `#[event]` from handler functions.
#[derive(Debug, Error)]
pub enum AttributeParseError {
    /// The pattern string in a `#[message]` attribute is empty
    #[error("Message Pattern Can not be empty")]
    MessagePatternIsEmpty,

    /// The pattern string in an `#[event]` attribute is empty
    #[error("Event Pattern Can not be empty")]
    EventPatternIsEmpty,

    /// An unknown attribute was encountered
    #[error("Unknown attribute")]
    UnknownAttribute,
}

impl Error {
    /// Convert the error into a compile error token stream
    ///
    /// This method generates a `compile_error!` macro call that will display
    /// the error message to the user during compilation.
    ///
    /// # Returns
    ///
    /// Returns a `TokenStream` containing a `compile_error!` macro invocation.
    pub fn write_errors(self) -> TokenStream {
        let message = self.to_string();
        quote! {
            compile_error!(#message)
        }
        .into()
    }
}
