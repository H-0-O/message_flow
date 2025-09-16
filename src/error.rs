use serde::{Deserialize, Serialize};

use crate::Message;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DefaultResponseErrorSchema {
    pub code: u16,
    pub message: String,
}

impl Message for DefaultResponseErrorSchema {}
