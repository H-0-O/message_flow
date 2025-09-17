use crate::{Message, error::DefaultResponseErrorSchema};
use async_nats::Client;
pub type ErrorHandler = fn(code: u16, message: String, client: &Client) -> Box<dyn Message>;

pub struct Context {
    pub error_handler: ErrorHandler,
    pub client: Client,
}

impl Context {
    pub fn new(client: Client) -> Context {
        Context {
            error_handler: |code, message, _client| {
                //here we should have error message or context as argument
                let error_response = DefaultResponseErrorSchema {
                    code,
                    message: message,
                };
                return Box::new(error_response);
            },
            client: client,
        }
    }
}
