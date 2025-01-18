use std::{any::type_name, io::Read, str::Bytes, sync::Arc};

use async_nats::{Client, ConnectOptions};
use futures::StreamExt;
use message_flow_drive::{event_pattern, msg_flow, msg_pattern, MsgDef};
use serde::{Deserialize, Serialize};
use serde_json::json;

macro_rules! append_register {
        ($($name:ident),*) => {
            [$( $name::register ),*]
        };
    }

pub type Result<T> = std::result::Result<T, async_nats::Error>;

#[tokio::test]
async fn main() {
    ::std::boxed::Box
    let register = append_register!(User, User2);
    // message_flow::connect("localhost:4222".into(), register);
    let e = connect().await;
}
fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}
pub async fn connect() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = ConnectOptions::new().connect("localhost:4222").await?;

    println!("CONNECTED");
    let registers = append_register!(User);
    print_type_of(&registers);
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
pub trait Message: MessageSerialize + Send {
    fn to_json(&self) -> String {
        self._to_json()
    }
}

pub trait MessageSerialize {
    fn _to_json(&self) -> String;
}

impl<T: Serialize + Message> MessageSerialize for T {
    fn _to_json(&self) -> String {
        let result = serde_json::to_string(self);
        match result {
            Ok(val) => val,
            Err(e) => panic!("OK ERROR"),
        }
    }
}

impl Message for () {}
impl Message for u128 {}
impl Message for String {}
impl Message for Vec<String> {}

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

trait Handler {
    const STRUCT_NAME: &str;
    async fn register(client: Arc<Client>) -> Result<()>;
    // async fn handle(&self, subject: String);
    async fn handle(subject: &String, payload: &[u8]) -> Result<Box<dyn Message>>;
}

impl Handler for User {
    const STRUCT_NAME: &str = "User";

    async fn register(client: Arc<Client>) -> Result<()> {
        println!("HELLO IN REGISTER");
        let mut requests = client.subscribe("service_A.*").await?;
        println!("THE subscribe IS WORKING");
        let handle = tokio::spawn({
            let client = client.clone();
            async move {
                // let resolver = User {
                //     first_name: "ff".into(),
                // };
                while let Some(request) = requests.next().await {
                    let result =
                        User::handle(&request.subject.to_string(), request.payload.as_ref()).await;

                    if let Err(err) = result {
                        return Err(err);
                    }

                    if let Some(reply) = request.reply {
                        let _ = client
                            .publish(reply, result.unwrap().to_json().into())
                            .await?;
                    }
                }
                Ok::<(), async_nats::Error>(())
            }
        });
        let _ = handle.await;
        Ok(())
    }

    async fn handle(subject: &String, payload: &[u8]) -> Result<Box<dyn Message>> {
        let resolver = serde_json::from_slice::<Self>(payload).unwrap();
        println!("IN HANDLE and message {:?} ", subject);
        let func: Box<dyn Message> = match subject.as_str() {
            "service_A.greeting" => Box::new(resolver.greeting().await?),
            "pattern_a" => Box::new(resolver.pattern_a().await?),
            "pattern_b" => Box::new(resolver.pattern_b().await?),
            "pattern_c" => Box::new(resolver.pattern_c().await?),
            _ => return Err(async_nats::Error::from("Pattern Not found")),
        };
        Ok(func)
    }
}

impl Handler for User2 {
    const STRUCT_NAME: &str = "User";

    async fn register(client: Arc<Client>) -> Result<()> {
        println!("HELLO IN REGISTER");
        let mut requests = client.subscribe("service_A.*").await?;
        println!("THE subscribe IS WORKING");
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
                        Err(error) => json!({
                            "error" : "Pattern Not Found"
                        })
                        .to_string(),
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
        Ok(Box::new(2))
    }
}
// #[msg_flow(prefix = "serviceA")]
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
