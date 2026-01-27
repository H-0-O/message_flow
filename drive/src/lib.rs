//! # message_flow_drive
//!
//! Procedural macros for the `message_flow` library.
//!
//! This crate provides the procedural macros that power the `message_flow` library:
//! - `#[derive(MsgDef)]`: Implements the `Message` trait for a struct
//! - `#[msg_flow]`: Marks an impl block as containing message handlers
//! - `#[message]`: Marks a method as a message handler (used within `#[msg_flow]` blocks)
//!
//! These macros are used internally by `message_flow` and should not be imported directly
//! by end users. Instead, use the re-exported macros from the `message_flow` crate.

use darling::FromMeta;
use error::Error;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod attribute;
mod constants;
mod error;
mod handler;
mod msg_def;
mod msg_flow;
mod register;

/// Parse nested meta attributes using darling
///
/// This macro simplifies parsing attribute arguments for proc macros.
/// It handles errors by converting them to compile errors.
macro_rules! parse_nested_meta {
    ($ty:ty, $args:expr) => {{
        let meta = match darling::ast::NestedMeta::parse_meta_list($args.into()) {
            Ok(v) => v,
            Err(err) => {
                return Error::from(err).write_errors().into();
            }
        };

        match <$ty>::from_list(&meta) {
            Ok(object_args) => object_args,
            Err(err) => return Error::from(err).write_errors().into(),
        }
    }};
}

/// Derive macro for implementing the `Message` trait
///
/// This macro automatically implements the `Message` trait for a struct, allowing
/// it to be used as a message type in message handlers.
///
/// ## Usage
///
/// ```rust,ignore
/// use message_flow::MsgDef;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(MsgDef, Serialize, Deserialize, Debug)]
/// struct MyMessage {
///     field: String,
/// }
/// ```
///
/// ## Requirements
///
/// The struct must also derive:
/// - `Serialize` and `Deserialize` from `serde`
/// - `Debug` from the standard library
///
/// ## Generated Code
///
/// This macro generates an implementation of `message_flow::Message` for the struct.
#[proc_macro_derive(MsgDef)]
pub fn msg_def(item: TokenStream) -> TokenStream {
    let __input = parse_macro_input!(item as syn::ItemStruct);

    let generated_tokens = msg_def::generate(__input);
    match generated_tokens {
        Ok(v) => v,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

/// Arguments for the `#[msg_flow]` attribute
#[derive(Debug, FromMeta)]
pub(crate) struct MsgFlowArgs {
    /// The base pattern/topic for all message handlers in this impl block
    #[darling(default)]
    pattern: Option<String>,
}

/// Attribute macro for defining message handlers
///
/// This macro marks an `impl` block as containing message handlers. It generates
/// the `Register` trait implementation that registers all handlers with the broker.
///
/// ## Usage
///
/// ```rust,ignore
/// use message_flow::{msg_flow, message, MsgDef};
///
/// #[derive(MsgDef, Serialize, Deserialize)]
/// struct MyHandler {
///     data: String,
/// }
///
/// #[msg_flow(pattern = "service_name")]
/// impl MyHandler {
///     #[message(pattern = "handler1")]
///     async fn handle1(&self) -> Result<String, Box<dyn std::error::Error>> {
///         Ok("Response".into())
///     }
/// }
/// ```
///
/// ## Arguments
///
/// - `pattern`: (optional) The base pattern/topic for all handlers in this impl block.
///              Individual handler patterns are appended to this base pattern.
///
/// ## Generated Code
///
/// This macro generates:
/// 1. An implementation of the `Register` trait
/// 2. Subscription code for each `#[message]` handler
/// 3. Task spawning for concurrent message handling
#[proc_macro_attribute]
pub fn msg_flow(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemImpl);
    let args = parse_nested_meta!(MsgFlowArgs, attr);
    let generated_tokens = msg_flow::generate(input, args);

    match generated_tokens {
        Ok(v) => v,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

/// Attribute macro for event handlers (placeholder)
///
/// This is a placeholder for future event handling functionality.
/// Currently, it does nothing and just passes through the function.
///
/// ## Future Usage
///
/// This macro will be used to mark functions as event handlers (fire-and-forget
/// pattern) as opposed to message handlers (request-reply pattern).
#[proc_macro_attribute]
pub fn event_pattern(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);

    quote! {#input}.into()
}
