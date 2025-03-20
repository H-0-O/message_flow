use futures::StreamExt;
// use message_flow::Handler;
// use message_flow::Register;
use message_flow::registers;
use message_flow::Message;
use message_flow::Result;
use message_flow_drive::{msg_flow, MsgDef};
use serde::{Deserialize, Serialize};

#[derive(MsgDef, Serialize, Deserialize, Debug)]
struct User {
    first_name: String,
}

#[tokio::test]
async fn main() {
    let _re = message_flow::connection::connect_and_wait("localhost:4222".into(), registers!(User))
        .await
        .unwrap();
    let usr = User {
        first_name: "HO".into(),
    };

    // println!("{:?}", usr.gr().await.unwrap());
}

#[msg_flow(pattern = "service_A")]
impl User {
    #[message(pattern = "service_A")]
    async fn greeting(&self) -> Result<String> {
        Ok("OK HEEELO FROM GREETING".into())
    }
}

//THIS IS A new idea
// the struct name without field can be used with non assoc methods in their impl block , and we don't parse the input for them and then
// it will be a dynamic schema for this
// #[derive(MsgDef)]
// struct Dynamic;

//this either can be with data: JsonVal or without it but it can't have self
//

// #[msg_flow(pattern = "Dynamic")]
// impl Dynamic {
//     #[message(pattern = "greeting")]
//     async fn greeting() {}
// }
