use std::fmt::Debug;

use serde::{Serialize, de::DeserializeOwned};


pub trait Message: MessageSerialize + Send + Debug {
    fn to_json(&self) -> String {
        self._to_json()
    }
}

pub trait MessageSerialize {
    fn _to_json(&self) -> String;
}

impl<T: Message + Serialize + DeserializeOwned + Send> MessageSerialize for T {
    fn _to_json(&self) -> String {
        let result = serde_json::to_string(self);
        match result {
            Ok(val) => val,
            Err(e) => panic!("Error happened {} " , e.is_data().to_string()),
        }
    }
}

impl Message for () {}
impl Message for u128 {}
impl Message for String {}
impl Message for Vec<String> {}
