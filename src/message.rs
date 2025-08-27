use std::fmt::Debug;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Debug, Serialize, Deserialize)]
pub struct InComeMessage<T>
where
    T: Serialize,
{
    // pub pattern: String,
    pub data: T,
    // pub id: String,
}

impl<T> InComeMessage<T>
where
    T: Serialize,
    T: DeserializeOwned,
    T: Debug,
{
    pub fn new(message: &[u8]) -> T {
        println!("OK IT's here ");
        let ee = serde_json::de::from_slice::<T>(message).unwrap();
        println!("OK THE EE {:?}", &ee);
        ee
    }
}

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
