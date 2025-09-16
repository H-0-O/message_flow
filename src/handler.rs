use std::sync::Arc;

use async_trait::async_trait;

use crate::{Context, Result};

#[async_trait]
pub trait Register {
    async fn register(client: Arc<Context>) -> Result<()>;
}
