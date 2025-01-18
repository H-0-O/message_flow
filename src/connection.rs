use std::{future::Future, pin::Pin, sync::Arc};

use async_nats::{Client, ConnectOptions};

use crate::{RegisterFn, Result};

pub async fn connect(
    addr: String,
    registers: &[RegisterFn],
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = ConnectOptions::new().connect(addr).await?;

    let arced = Arc::new(client);
    for func in registers {
        if let Err(error) = func(arced.clone()).await {
            return Err(error);
        }
    }
    Ok(())
}
