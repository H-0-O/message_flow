use futures::StreamExt;
use message_flow::{registers, Client, Context, Result};
use message_flow_drive::{MsgDef, msg_flow};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(MsgDef, Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: Option<u128>,
}

#[derive(MsgDef , Serialize, Deserialize, Debug)]
struct ErrorResponse {
    message: String
}

#[tokio::test]
async fn main() {
    let app = message_flow::app::App::new("localhost:4222".into())
        .await
        .unwrap()
        .set_error_handler(|code: u16, message: String, client: &Client|{
            let rr= ErrorResponse {
                message: String::from("THIS IS TEST ERROR MESSAGE")
            };
            return Box::new(rr);
        })
        .connect_and_wait(registers!(User))
        .await;
}

#[derive(MsgDef, Debug, Serialize, Deserialize)]
struct UserResponse {
    family_name: String,
}

#[msg_flow]
impl User {
    #[message(pattern = "error")]
    async fn greeting(&self , ctx: &Context) -> Result<UserResponse> {
        let payload = json! (
            {
                "name": "Hossein"
            }
        ).to_string().into();
        let  res = ctx.client.publish("from_here", payload).await;
        Err(async_nats::Error::from("Error happened"))
    }

    #[message(pattern = "user_response")]
    async fn ok(self) -> Result<UserResponse> {
        let user_response = UserResponse {
            family_name: format!("{:} {}", self.name, " Edd"),
        };

        Ok(user_response)
    }
    #[event(pattern = "log")]
    async fn log(&self) {
        println!("A log printed here");
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
