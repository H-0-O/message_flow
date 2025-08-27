use message_flow::{registers, Result};
use message_flow_drive::{msg_flow, MsgDef};
use serde::{Deserialize, Serialize};
use futures::StreamExt;

#[derive(MsgDef, Serialize, Deserialize, Debug)]
struct User {
    name: String,
}


#[tokio::test]
async fn main() {

    let _re = message_flow::connection::connect_and_wait("localhost:4222".into(), registers!(User))
        .await
        .unwrap();

}

#[derive(MsgDef , Debug , Serialize , Deserialize)]
struct  UserResponse {
    family_name: String
}

#[msg_flow]
impl User {
    #[message(pattern = "PA")]
    async fn greeting(&self) -> Result<UserResponse> {
        println!("IN SERVICE 1");
        let user_response = UserResponse {
            family_name: "Hossein Salehi".into()
        };

        Ok(user_response)
    }

    // #[message(pattern = "PA")]
    // async fn greeting2(&self) -> Result<String> {
    //     println!("IN SERVICE 1");
    //     Ok("OK HEEELO FROM GREETING".into())
    // }

    // #[message(pattern = "3")]
    // async fn greeting3(&self) -> Result<String> {
    //     println!("IN SERVICE 1");
    //     Ok("OK HEEELO FROM GREETING".into())
    // }
    // #[event(pattern = "service_B")]
    // async fn ok(&self){
    //     println!("SERVICE B IS READY");
    // }
}


#[derive(MsgDef, Serialize, Deserialize, Debug)]
struct User2 {
    first_name: String,
}


// #[msg_flow(pattern = "service_B")]
// impl User2 {
//     #[message(pattern = "ss")]
//     async fn greeting(&self) -> Result<String> {
//         println!("IN SERVICE 1");
//         Ok("OK HEEELO FROM GREETING".into())
//     }

//     #[message(pattern = "service_2")]
//     async fn ok(&self) -> Result<String>{
//         println!("SERVICE B IS READY");
//         Ok("OK HEEELO FROM GREETING".into())
//     }
// }

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