//! Handler registration trait
//!
//! The [`Register`] trait is automatically implemented by the `#[msg_flow]` macro
//! for types that define message handlers.

use std::sync::Arc;

use async_trait::async_trait;

use crate::{Context, Result};

/// Trait for types that can register message handlers with the broker
///
/// This trait is automatically implemented by the `#[msg_flow]` procedural macro.
/// Types that derive `MsgDef` and have an `impl` block annotated with `#[msg_flow]`
/// will automatically implement this trait.
///
/// The `register` method is called during application startup to set up subscriptions
/// for all message patterns defined in the handler implementation.
///
/// ## Example
///
/// ```rust,no_run
/// use futures::StreamExt;
/// use message_flow::{msg_flow, MsgDef, Register, Message, Result};
/// use serde::{Serialize, Deserialize};
///
/// #[derive(MsgDef, Serialize, Deserialize, Debug)]
/// struct MyHandler {
///     data: String,
/// }
///
/// #[msg_flow(pattern = "service")]
/// impl MyHandler {
///     #[message(pattern = "handler")]
///     async fn handle(&self) -> Result<String> {
///         Ok("Success".into())
///     }
/// }
///
/// // MyHandler now implements Register automatically
/// ```
#[async_trait]
pub trait Register {
    /// Register all message handlers for this type with the broker
    ///
    /// This method sets up subscriptions for all message patterns defined in the
    /// `#[msg_flow]` impl block and spawns tasks to handle incoming messages.
    ///
    /// # Arguments
    ///
    /// * `client` - An `Arc<Context>` containing the broker client and error handler
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all handlers are registered successfully, or an error
    /// if registration fails.
    async fn register(client: Arc<Context>) -> Result<()>;
}
