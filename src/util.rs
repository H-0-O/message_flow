use std::{future::Future, pin::Pin, sync::Arc};

use async_nats::Client;

use crate::Result;

#[macro_export]
macro_rules! registers {
    ($($handler:ty),* $(,)?) => {
        &[
            $(<$handler>::register as RegisterFn),*
        ]
    };
}

pub type RegisterFn = fn(Arc<Client>) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;
