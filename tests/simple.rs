use async_nats::connect;
use futures::StreamExt;
use message_flow::Result;
use message_flow_drive::MsgDef;
use serde::{Deserialize, Serialize};

#[tokio::test]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = connect("localhost:4222").await?;
    let mut subscriber = client.subscribe("service_A.1").await?;

    while let Some(msg) = subscriber.next().await {
        // let payload = std::str::from_utf8()?;
        // let resolver = message_flow::InComeMessage::<User>::new(msg.payload.as_ref());
        let __resolver = serde_json::de::from_slice::<User>(msg.payload.as_ref());
        let resolver = match __resolver {
            Ok(t) => t,
            Err(e) => {
                let e = e.to_string();
                panic!("dijfoi")
            }
        };
        let result = ::std::boxed::Box::new(resolver.test_1().await);

        if let Err(e) = *result {
            return Err("Error for now".into());
        };
        if let Some(reply) = msg.reply {
            // message_flow::logger::info_log!(
            //     "Sending reply for struct {}: {:?}",
            //     stringify!(#struct_name),
            //     reply
            // );
            let _ = client.publish(reply, result.unwrap().into()).await?;
        }
    }
    Ok(())
}

#[derive(MsgDef, Serialize, Deserialize, Debug)]
struct User {
    name: String,
}

impl User {
    async fn test_1(&self) -> Result<String> {
        println!("in test 1 {:?}", self);

        Ok("HELLO".into())
    }
}
