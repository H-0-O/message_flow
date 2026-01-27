use serde::{Deserialize, Serialize};

use crate::Message;

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultResponseErrorSchema {
    pub code: u16,
    pub message: String,
}

impl Message for DefaultResponseErrorSchema {}
