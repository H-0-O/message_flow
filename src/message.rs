use serde::{de::DeserializeOwned, Serialize};

pub trait Message: MessageSerialize {
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
            Err(e) => panic!("OK ERROR"),
        }
    }
}

impl Message for () {}
impl Message for u128 {}
impl Message for String {}
impl Message for Vec<String> {}
