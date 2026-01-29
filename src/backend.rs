//! Internal broker abstraction for NATS and RabbitMQ backends.
//!
//! This module hides the concrete broker client types behind a small
//! API that is used by the generated code from `message_flow_drive`.
//! Only **one** backend is compiled in at a time, controlled by Cargo
//! features (`nats` or `rabbitmq`).

use bytes::Bytes;

use crate::Context;

/// Unified error type for broker operations.
#[cfg(feature = "nats")]
pub type BrokerError = async_nats::Error;

#[cfg(feature = "rabbitmq")]
pub type BrokerError = lapin::Error;

/// Result type for broker operations.
pub type BrokerResult<T> = std::result::Result<T, BrokerError>;

/// Unified message type used by the generated handler code.
pub struct BrokerMessage {
    /// Raw payload bytes.
    pub payload: Bytes,
    /// Optional reply subject / queue.
    pub reply: Option<String>,
}

/// Unified subscription/consumer wrapper.
pub struct BrokerSubscription {
    #[cfg(feature = "nats")]
    inner: async_nats::Subscriber,

    #[cfg(feature = "rabbitmq")]
    inner: lapin::Consumer,
}

impl BrokerSubscription {
    /// Receive the next message from the subscription.
    pub async fn next(&mut self) -> Option<BrokerMessage> {
        #[cfg(feature = "nats")]
        {
            use futures::StreamExt;

            self.inner.next().await.map(|msg| BrokerMessage {
                payload: Bytes::from(msg.payload),
                reply: msg.reply.map(|s| s.to_string()),
            })
        }

        #[cfg(feature = "rabbitmq")]
        {
            use futures::StreamExt;

            self.inner.next().await.map(|delivery| {
                let (_, delivery) = delivery.expect("delivery error");
                let payload = Bytes::from(delivery.data.clone());
                let reply = delivery
                    .properties
                    .reply_to()
                    .as_ref()
                    .and_then(|s| s.as_str().map(|s| s.to_string()));

                BrokerMessage { payload, reply }
            })
        }
    }
}

/// Subscribe to a topic/queue based on the active backend.
pub async fn subscribe(context: &Context, pattern: &str) -> BrokerResult<BrokerSubscription> {
    #[cfg(feature = "nats")]
    {
        let subscriber = context.client.subscribe(pattern.to_string()).await?;
        Ok(BrokerSubscription { inner: subscriber })
    }

    #[cfg(feature = "rabbitmq")]
    {
        use lapin::{options::*, types::FieldTable};

        // For RabbitMQ, we treat `pattern` as a queue name and create/consume it.
        let channel = &context.client;
        channel
            .queue_declare(
                pattern,
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        let consumer = channel
            .basic_consume(
                pattern,
                "",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(BrokerSubscription { inner: consumer })
    }
}

/// Publish a reply message based on the active backend.
pub async fn publish_reply(context: &Context, reply: &str, body: Bytes) -> BrokerResult<()> {
    #[cfg(feature = "nats")]
    {
        context.client.publish(reply.to_string(), body).await?;
        Ok(())
    }

    #[cfg(feature = "rabbitmq")]
    {
        use lapin::{options::*, BasicProperties};

        let channel = &context.client;
        channel
            .basic_publish(
                "",
                reply,
                BasicPublishOptions::default(),
                body.to_vec(),
                BasicProperties::default(),
            )
            .await?
            .await?;

        Ok(())
    }
}


