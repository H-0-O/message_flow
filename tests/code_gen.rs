use futures::StreamExt;
// use message_flow::Handler;
// use message_flow::Register;
use message_flow::{registers, Result};
use message_flow_drive::{msg_flow, MsgDef};
use serde::{Deserialize, Serialize};

#[derive(MsgDef, Serialize, Deserialize)]
struct User {
    first_name: String,
}

#[tokio::test]
async fn main() {
    let _re = message_flow::connection::connect("".into(), registers!(User)).await;
    let usr = User {
        first_name: "HO".into(),
    };
    // println!("{:?}", usr.gr().await.unwrap());
}

#[msg_flow(pattern = "user")]
impl User {
    #[msg_pattern(pattern = "ee")]
    async fn greeting(&self) -> Result<String> {
        Ok("OK HEEELO FROM GREETING".into())
    }
}

// #[msg(pattern = "ee")]
// async fn gr(&self) -> Result<String> {
//     Ok("OK HEEELO FROM gr".into())
// }

// #[event(pattern = "once")]
// async fn gr(&self) -> Result<String> {
//     Ok("OK HEEELO FROM gr".into())
// }
