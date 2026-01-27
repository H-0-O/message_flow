//! Application initialization and connection management
//!
//! The [`App`] struct is the main entry point for creating and managing message flow
//! applications. It handles broker connections and handler registration.

use std::sync::Arc;

use async_nats::ConnectOptions;

use crate::context::ErrorHandler;
use crate::{RegisterFn, context::Context, logger};

/// Main application struct for managing message flow connections
///
/// The `App` struct encapsulates the connection to the message broker and provides
/// methods to register handlers and manage the application lifecycle.
///
/// ## Example
///
/// ```rust,no_run
/// use message_flow::{App, registers};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let app = App::new("localhost:4222".to_string()).await?;
/// app.connect_and_wait(registers!(MyHandler)).await?;
/// # Ok(())
/// # }
/// ```
pub struct App {
    /// The application context containing the broker client and error handler
    pub context: Context
}

impl App {
    /// Create a new `App` instance and connect to the message broker
    ///
    /// # Arguments
    ///
    /// * `addr` - The address of the message broker (e.g., "localhost:4222" for NATS)
    ///
    /// # Returns
    ///
    /// Returns `Ok(App)` if the connection is successful, or an error if the connection fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use message_flow::App;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let app = App::new("localhost:4222".to_string()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(addr: String) -> std::result::Result<App, Box<dyn std::error::Error>> {
        let client = ConnectOptions::new().connect(&addr).await?;
        let app = App {
            context: Context::new(client)
        };
        Ok(app)
    }

    /// Register handlers and connect to the broker
    ///
    /// This method registers all provided handlers with the broker and starts listening
    /// for messages. The function returns immediately after registration.
    ///
    /// # Arguments
    ///
    /// * `registers` - A slice of registration functions, typically created with the `registers!` macro
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all handlers are registered successfully, or an error if registration fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use message_flow::{App, registers};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let app = App::new("localhost:4222".to_string()).await?;
    /// app.connect(registers!(MyHandler1, MyHandler2)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect(
        self,
        registers: &[RegisterFn],
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "nats")]
        logger::info_log!("Connected to NATS server");

        let arced: Arc<Context> = Arc::new(self.context);
        for func in registers {
            if let Err(error) = func(arced.clone()).await {
                return Err(error);
            }
        }
        Ok(())
    }

    /// Register handlers, connect to the broker, and wait for shutdown signal
    ///
    /// This method combines `connect` with a signal handler that waits for Ctrl+C
    /// before shutting down. This is the recommended way to run a long-running service.
    ///
    /// # Arguments
    ///
    /// * `registers` - A slice of registration functions, typically created with the `registers!` macro
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when the application shuts down gracefully, or an error if registration fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use message_flow::{App, registers};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let app = App::new("localhost:4222".to_string()).await?;
    /// app.connect_and_wait(registers!(MyHandler)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect_and_wait(
        self,
        registers: &[RegisterFn],
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let _ = self.connect(registers).await?;
        tokio::signal::ctrl_c().await?;
        logger::info_log!("Program comes to end");
        Ok(())
    }

    /// Set a custom error handler for the application
    ///
    /// By default, the application uses a standard error response format. This method
    /// allows you to customize how errors are formatted and sent back to clients.
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that takes an error code, message, and client, and returns
    ///                a boxed `Message` to send as the error response
    ///
    /// # Returns
    ///
    /// Returns `Self` for method chaining.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use message_flow::{App, Message, DefaultResponseErrorSchema};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let app = App::new("localhost:4222".to_string()).await?
    ///     .set_error_handler(|code, msg, _client| {
    ///         Box::new(DefaultResponseErrorSchema {
    ///             code,
    ///             message: format!("Custom error: {}", msg),
    ///         })
    ///     });
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_error_handler(mut self, callback: ErrorHandler) -> Self {
        self.context.error_handler = callback;
        self
    }
}
