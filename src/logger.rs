//! Feature-gated logging macros
//!
//! This module provides logging macros that are conditionally compiled based on
//! feature flags. This allows you to control logging at compile time for better
//! performance in production builds.

/// Debug-level logging macro
///
/// Logs a debug message if both the `log` and `debug` features are enabled.
///
/// ## Usage
///
/// ```rust
/// use message_flow::debug_log;
///
/// debug_log!("Debug message: {}", value);
/// ```
///
/// ## Features
///
/// Requires both `log` and `debug` features to be enabled.
#[macro_export]
macro_rules! debug_log {
    ($($args:tt)*) => {
        #[cfg(all(feature = "log" , feature = "debug"))]
        log::debug!($($args)*);
    };
}

/// Info-level logging macro
///
/// Logs an info message if both the `log` and `info` features are enabled.
///
/// ## Usage
///
/// ```rust
/// use message_flow::info_log;
///
/// info_log!("Info message: {}", value);
/// ```
///
/// ## Features
///
/// Requires both `log` and `info` features to be enabled.
#[macro_export]
macro_rules! info_log {
    ($($args:tt)*) => {
        #[cfg(all(feature = "log" , feature = "info"))]
        log::info!($($args)*);
    };
}

/// Warning-level logging macro
///
/// Logs a warning message if both the `log` and `warn` features are enabled.
///
/// ## Usage
///
/// ```rust
/// use message_flow::warn_log;
///
/// warn_log!("Warning message: {}", value);
/// ```
///
/// ## Features
///
/// Requires both `log` and `warn` features to be enabled.
#[macro_export]
macro_rules! warn_log {
    ($($args:tt)*) => {
        #[cfg(all(feature = "log" , feature = "warn"))]
        log::warn!($($args)*);
    };
}

/// Error-level logging macro
///
/// Logs an error message if both the `log` and `error` features are enabled.
///
/// ## Usage
///
/// ```rust
/// use message_flow::error_log;
///
/// error_log!("Error message: {}", value);
/// ```
///
/// ## Features
///
/// Requires both `log` and `error` features to be enabled.
#[macro_export]
macro_rules! error_log {
    ($($args:tt)*) => {
        #[cfg(all(feature = "log" , feature = "error"))]
        log::error!($($args)*);
    };
}

/// Trace-level logging macro
///
/// Logs a trace message if both the `log` and `trace` features are enabled.
///
/// ## Usage
///
/// ```rust
/// use message_flow::trace_log;
///
/// trace_log!("Trace message: {}", value);
/// ```
///
/// ## Features
///
/// Requires both `log` and `trace` features to be enabled.
#[macro_export]
macro_rules! trace_log {
    ($($args:tt)*) => {
        #[cfg(all(feature = "log" , feature = "trace"))]
        log::trace!($($args)*);
    };
}

// Re-export macros for convenience
pub use debug_log;
pub use info_log;
pub use warn_log;
pub use error_log;
pub use trace_log;