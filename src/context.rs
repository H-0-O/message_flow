//! Request context and error handling
//!
//! The [`Context`] struct provides access to the message broker client and error
//! handling functionality for message handlers.

use crate::error::DefaultResponseErrorSchema;
use async_nats::Client;

/// Type alias for error handler functions
///
/// Error handlers are called when a message handler encounters an error. They receive:
/// - `code`: HTTP-style error code (e.g., 500 for server errors, 422 for validation errors)
/// - `message`: Error message string
/// - `client`: Reference to the NATS client (for potential custom error handling)
///
/// The handler must return a boxed error response that can be serialized.
/// 
/// Note: Due to `Message` having an associated const (`IS_UNIT`), it is not object-safe.
/// We use a concrete type (`DefaultResponseErrorSchema`) for error responses, which implements `Message`.
pub type ErrorHandler = fn(code: u16, message: String, client: &Client) -> DefaultResponseErrorSchema;

/// Application context passed to message handlers
///
/// The `Context` struct provides handlers with access to:
/// - The message broker client for sending messages
/// - The error handler for customizing error responses
///
/// Handlers can optionally accept a `&Context` parameter to access these resources.
///
/// ## Example
///
/// ```rust,no_run
/// use message_flow::{msg_flow, message, Context, MsgDef};
/// use serde::{Serialize, Deserialize};
///
/// #[derive(MsgDef, Serialize, Deserialize)]
/// struct MyMessage {
///     data: String,
/// }
///
/// #[msg_flow(pattern = "service")]
/// impl MyMessage {
///     #[message(pattern = "handler")]
///     async fn handle(&self, context: &Context) -> Result<String, Box<dyn std::error::Error>> {
///         // Use context.client to send messages
///         Ok("Success".into())
///     }
/// }
/// ```
pub struct Context {
    /// Function called when a handler encounters an error
    pub error_handler: ErrorHandler,
    /// The NATS client for sending messages
    pub client: Client,
}

impl Context {
    /// Create a new `Context` with default error handling
    ///
    /// The default error handler uses `DefaultResponseErrorSchema` to format errors.
    ///
    /// # Arguments
    ///
    /// * `client` - A connected NATS client
    ///
    /// # Returns
    ///
    /// Returns a new `Context` instance with default error handling.
    pub fn new(client: Client) -> Context {
        Context {
            error_handler: |code, message, _client| {
                DefaultResponseErrorSchema {
                    code,
                    message: message,
                }
            },
            client: client,
        }
    }
}
