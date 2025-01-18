use async_nats::{Client, ConnectOptions};
use async_trait::async_trait;
use message_flow::{registers, Message, Result};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let re: &[RegisterFn] = registers!(User, User2);
    let client = ConnectOptions::new()
        .connect("localhost:4222")
        .await
        .unwrap();
    let client_arc = Arc::new(client);

    // Execute tasks concurrently
    let futures: Vec<_> = re.iter().map(|f| f(client_arc.clone())).collect();
    futures::future::join_all(futures).await;
}

// Define a boxed async function type

pub type RegisterFn = fn(Arc<Client>) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;

// Wrap async functions to be stored in a slice
// fn wrap_register<F, Fut>(func: F) -> RegisterFn
// where
//     F: Fn(Arc<Client>) -> Fut + Send + Sync + 'static,
//     Fut: Future<Output = Result<()>> + Send + 'static,
// {
//     Box::new(move |client: Arc<Client>| Box::pin(func(client)))
// }
// Sample async functions
#[async_trait]
trait Handler {
    const STRUCT_NAME: &str;
    async fn register(client: Arc<Client>) -> Result<()>;
    // async fn handle(&self, subject: String);
    async fn handle(&self, subject: &String) -> Result<Box<dyn Message>>;
}

struct User {
    fi: String,
}

struct User2 {
    fi: String,
}

#[async_trait]
impl Handler for User {
    const STRUCT_NAME: &str = "User";

    async fn register(client: Arc<Client>) -> Result<()> {
        Ok(())
    }

    async fn handle(&self, subject: &String) -> Result<Box<dyn Message>> {
        todo!()
    }
}

#[async_trait]
impl Handler for User2 {
    const STRUCT_NAME: &str = "User2";

    async fn register(client: Arc<Client>) -> Result<()> {
        Ok(())
    }

    async fn handle(&self, subject: &String) -> Result<Box<dyn Message>> {
        todo!()
    }
}
