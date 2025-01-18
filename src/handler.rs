use std::sync::Arc;

use async_nats::Client;
use async_trait::async_trait;

use crate::{Message, Result};

#[async_trait]
pub trait Register {
    async fn register(client: Arc<Client>) -> Result<()>;
}

#[async_trait]
pub trait Handler {
    async fn router(subject: &String, payload: &[u8]) -> Result<Box<dyn Message>>;
}
