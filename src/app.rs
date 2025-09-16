use std::sync::Arc;

use async_nats::ConnectOptions;

use crate::context::ErrorHandler;
use crate::{RegisterFn, context::Context, logger};


pub struct App {
    pub context: Context
}

impl App {
    pub async fn new(addr: String) -> std::result::Result<App, Box<dyn std::error::Error>> {
        let client = ConnectOptions::new().connect(&addr).await?;
        let app = App {
            context: Context::new(client)
        };
        Ok(app)
    }

    pub async fn connect(
        self,
        registers: &[RegisterFn],
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "nats")]
        logger::info_log!("Connected to NATS server at {}", &addr);

        let arced: Arc<Context> = Arc::new(self.context);
        for func in registers {
            if let Err(error) = func(arced.clone()).await {
                return Err(error);
            }
        }
        Ok(())
    }

    pub async fn connect_and_wait(
        self,
        registers: &[RegisterFn],
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let _ = self.connect(registers).await?;
        tokio::signal::ctrl_c().await?;
        logger::info_log!("Program comes to end");
        Ok(())
    }

    pub fn set_error_handler(mut self, callback: ErrorHandler) -> Self {
        self.context.error_handler = callback;
        self
    }
}
