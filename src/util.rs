use std::{future::Future, pin::Pin, sync::Arc};

use async_nats::Client;

use crate::Result;

pub type RegisterFn = fn(Arc<Client>) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;

#[macro_export]
macro_rules! registers {
    ($($handler:ty),* $(,)?) => {
        {
             use message_flow::Register;
            &[
                $(<$handler>::register as message_flow::RegisterFn),*
            ]
        }
    };
}
