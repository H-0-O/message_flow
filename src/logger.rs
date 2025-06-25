
#[macro_export]
macro_rules! debug_log {
    ($($args:tt)*) => {
        #[cfg(all(feature = "log" , feature = "debug"))]
        log::debug!($($args)*);
    };
}

#[macro_export]
macro_rules! info_log {
    ($($args:tt)*) => {
        #[cfg(all(feature = "log" , feature = "info"))]
        log::info!($($args)*);
    };
}

#[macro_export]
macro_rules! warn_log {
    ($($args:tt)*) => {
        #[cfg(all(feature = "log" , feature = "warn"))]
        log::warn!($($args)*);
    };
}

#[macro_export]
macro_rules! error_log {
    ($($args:tt)*) => {
        #[cfg(all(feature = "log" , feature = "error"))]
        log::error!($($args)*);
    };
}

#[macro_export]
macro_rules! trace_log {
    ($($args:tt)*) => {
        #[cfg(all(feature = "log" , feature = "trace"))]
        log::trace!($($args)*);
    };
}

pub use debug_log;
pub use info_log;
pub use warn_log;
pub use error_log;
pub use trace_log;