//! Result type aliases
//!
//! This module provides convenient type aliases for common result types used
//! throughout the message_flow library.

/// Result type alias for handler operations
///
/// This is a convenience alias for `std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>`.
/// It's used throughout the library for operations that interact with the
/// underlying message broker (NATS or RabbitMQ) or other fallible logic.
///
/// ## Example
///
/// ```rust
/// use message_flow::Result;
///
/// async fn example() -> Result<()> {
///     // Handler and broker operations return this Result type
///     Ok(())
/// }
/// ```
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
