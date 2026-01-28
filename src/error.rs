//! Error response types
//!
//! This module provides default error response schemas for message handlers.

use serde::{Deserialize, Serialize};

use crate::Message;

/// Default error response schema used by the application
///
/// This struct represents the standard error response format sent when a message
/// handler encounters an error. It includes an HTTP-style status code and an error message.
///
/// ## Example
///
/// ```rust
/// use message_flow::DefaultResponseErrorSchema;
///
/// let error = DefaultResponseErrorSchema {
///     code: 500,
///     message: "Internal server error".to_string(),
/// };
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultResponseErrorSchema {
    /// HTTP-style error code (e.g., 400, 404, 500)
    pub code: u16,
    /// Human-readable error message
    pub message: String,
}

impl Message for DefaultResponseErrorSchema {
    const IS_UNIT: bool = false;
}
