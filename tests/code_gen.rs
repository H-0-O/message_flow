use futures::StreamExt;
// use message_flow::Handler;
// use message_flow::Register;
use message_flow::registers;
use message_flow::Message;
use message_flow::Result;
use message_flow_drive::event_pattern;
use message_flow_drive::{msg_flow, MsgDef};
use serde::{Deserialize, Serialize};

#[derive(MsgDef, Serialize, Deserialize, Debug)]
struct User {
    first_name: String,
}


#[tokio::test]
async fn main() {
    println!("BEFORE ");

    let _re = message_flow::connection::connect_and_wait("localhost:4222".into(), registers!(User , User2))
        .await
        .unwrap();
    let usr = User {
        first_name: "HO".into(),
    };

}

#[msg_flow(pattern = "service_A")]
impl User {
    #[message(pattern = "1")]
    async fn greeting(&self) -> Result<String> {
        println!("IN SERVICE 1");
        Ok("OK HEEELO FROM GREETING".into())
    }

    #[message(pattern = "2")]
    async fn greeting2(&self) -> Result<String> {
        println!("IN SERVICE 1");
        Ok("OK HEEELO FROM GREETING".into())
    }

    #[message(pattern = "3")]
    async fn greeting3(&self) -> Result<String> {
        println!("IN SERVICE 1");
        Ok("OK HEEELO FROM GREETING".into())
    }
    // #[event(pattern = "service_B")]
    // async fn ok(&self){
    //     println!("SERVICE B IS READY");
    // }
}


#[derive(MsgDef, Serialize, Deserialize, Debug)]
struct User2 {
    first_name: String,
}


#[msg_flow(pattern = "service_B")]
impl User2 {
    #[message(pattern = "ss")]
    async fn greeting(&self) -> Result<String> {
        println!("IN SERVICE 1");
        Ok("OK HEEELO FROM GREETING".into())
    }

    #[message(pattern = "service_2")]
    async fn ok(&self) -> Result<String>{
        println!("SERVICE B IS READY");
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


//The return types must be Result , it must checks at compile time and throw error if 
// user use anything else instead of result or we must adapt the generated code with the user return