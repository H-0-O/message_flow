pub mod connection;
mod handler;
mod message;
pub mod result;
pub mod util;

pub use handler::{Handler, Register};
pub use message::Message;
pub use result::Result;
pub use util::*;

pub use async_nats::Client;
pub use async_trait::async_trait;
#[cfg(test)]
mod tests {
    use core::panic;
    use std::{collections::HashMap, error::Error, future::Future};

    use async_nats::Client;
    use message_flow_drive::{event_pattern, msg_flow, msg_pattern, MsgDef};
    use serde::{
        de::{DeserializeOwned, IntoDeserializer},
        Deserialize, Serialize, Serializer,
    };

    use super::*;

    #[tokio::test]
    async fn main() {
        let _ = connect().await;
    }

    macro_rules! append_register {
        ($($name:ident),*) => {
            [$( $name::register ),*]
        };
    }
    pub async fn connect() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let client = ConnectOptions::new().connect("localhost:4222").await?;

        let registers = append_register!(User);
        let arced = Arc::new(client);
        for func in registers {
            if let Err(error) = func(arced.clone()).await {
                return Err(error);
            }
        }
        Ok(())
    }

    // async fn run_h(client: Client) -> std::result::Result<(), Box<dyn std::error::Error>> {}

    // pub type Handler = Arc<dyn Fn() -> BoxFuture<'static, Box<dyn Message + Send>> + Send + Sync>;
    #[MsgDef]
    #[derive(Serialize, Deserialize)]
    struct User {
        first_name: String,
    }

    impl Message for User {}
    impl Message for User2 {}

    #[MsgDef]
    #[derive(Serialize, Deserialize)]
    struct User2 {
        last_name: String,
    }

    type Result<T> = std::result::Result<T, async_nats::Error>;

    trait Handler {
        const STRUCT_NAME: &str;
        async fn register(client: Arc<Client>) -> Result<()>;
        // async fn handle(&self, subject: String);
        async fn handle(&self, subject: &String) -> Result<Box<dyn Message>>;
    }

    impl Handler for User {
        const STRUCT_NAME: &str = "User";

        async fn register(client: Arc<Client>) -> Result<()> {
            let mut requests = client.subscribe("service_A.*").await?;

            let handle = tokio::spawn({
                let client = client.clone();
                async move {
                    let resolver = User {
                        first_name: "ff".into(),
                    };
                    while let Some(request) = requests.next().await {
                        let result = resolver.handle(&request.subject.to_string()).await;
                        let un = match result {
                            Ok(val) => val._to_json(),
                            Err(error) => error.to_string(),
                        };
                        if let Some(reply) = request.reply {
                            let _ = client.publish(reply, un.into()).await?;
                        }
                    }
                    Ok::<(), async_nats::Error>(())
                }
            });
            let _ = handle.await;
            Ok(())
        }

        async fn handle(&self, subject: &String) -> Result<Box<dyn Message>> {
            let func: Box<dyn Message> = match subject.as_str() {
                "greeting" => Box::new(self.greeting().await?),
                "pattern_a" => Box::new(self.pattern_a().await?),
                "pattern_b" => Box::new(self.pattern_b().await?),
                "pattern_c" => Box::new(self.pattern_c().await?),
                _ => return Err(async_nats::Error::from("Pattern Not found")),
            };
            Ok(func)
        }
    }

    #[msg_flow(prefix = "serviceA")]
    impl User {
        #[msg_pattern(pattern = "greeting")]
        async fn greeting(&self) -> Result<User> {
            Ok(User {
                first_name: "HKH".into(),
            })
        }

        #[msg_pattern(pattern = "pattern_a")]
        async fn pattern_a(&self) -> Result<User2> {
            let func = || User2 {
                last_name: "Hello2".into(),
            };
            Ok(func())

            // Box::new(func())
            // User {
            //     first_name: "H".into(),
            // }
        }

        #[msg_pattern(pattern = "pattern_b")]
        async fn pattern_b(&self) -> Result<()> {
            Ok(())
            // Box::new(func())
        }

        #[event_pattern(pattern = "pattern_c")]
        async fn pattern_c(&self) -> Result<Vec<String>> {
            Ok(Vec::new())
            // Box::new(func())
        }
    }
}
