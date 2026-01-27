//! Result type aliases
//!
//! This module provides convenient type aliases for common result types used
//! throughout the message_flow library.

/// Result type alias for NATS operations
///
/// This is a convenience alias for `std::result::Result<T, async_nats::Error>`.
/// It's used throughout the library for operations that interact with the NATS broker.
///
/// ## Example
///
/// ```rust
/// use message_flow::Result;
///
/// async fn example() -> Result<()> {
///     // NATS operations return this Result type
///     Ok(())
/// }
/// ```
pub type Result<T> = std::result::Result<T, async_nats::Error>;
