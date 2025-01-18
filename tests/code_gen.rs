use futures::StreamExt;
use message_flow::registers;
use message_flow_drive::{msg_flow, MsgDef};
use serde::{Deserialize, Serialize};

#[tokio::test]
async fn main() {
    let re = message_flow::connection::connect("".into(), registers!(User)).await;
}

#[derive(MsgDef, Serialize, Deserialize)]
struct User {}

#[msg_flow(pattern = "user")]
impl User {
    async fn greeting(&self) {}
}
