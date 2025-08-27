use std::{sync::Arc};

use async_nats::{ConnectOptions};

use crate::{logger, RegisterFn};

pub async fn connect(
    addr: String,
    registers: &[RegisterFn],
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = ConnectOptions::new().connect(&addr).await?;
    #[cfg(feature = "nats")]
    logger::info_log!("Connected to NATS server at {}", &addr);

    let arced = Arc::new(client);
    for func in registers {
        if let Err(error) = func(arced.clone()).await {
            return Err(error);
        }
    }
    Ok(())
}

pub async fn connect_and_wait(
    addr: String,
    registers: &[RegisterFn],
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let _ = connect(addr, registers).await?;
    tokio::signal::ctrl_c().await?;
    logger::info_log!("Program comes to end");
    Ok(())
}
