//! Message serialization trait
//!
//! The [`Message`] trait defines types that can be serialized and sent as messages
//! through the message broker.

use std::fmt::Debug;

use serde::{Serialize, de::DeserializeOwned};

/// Trait for types that can be serialized as messages
///
/// Types implementing `Message` can be returned from message handlers and will be
/// automatically serialized to JSON before being sent over the message broker.
///
/// The trait is automatically implemented for types that:
/// - Implement `Serialize` and `DeserializeOwned` from `serde`
/// - Implement `Send` and `Debug`
///
/// The `#[derive(MsgDef)]` macro automatically implements this trait for your types.
///
/// ## Example
///
/// ```rust
/// use message_flow::{Message, MsgDef};
/// use serde::{Serialize, Deserialize};
///
/// #[derive(MsgDef, Serialize, Deserialize, Debug)]
/// struct MyResponse {
///     status: String,
///     data: i32,
/// }
///
/// // MyResponse now implements Message
/// let response = MyResponse {
///     status: "ok".to_string(),
///     data: 42,
/// };
/// let json = response.to_json();
/// ```
pub trait Message: MessageSerialize + Send + Debug {
    /// Serialize the message to a JSON string
    ///
    /// This method converts the message to a JSON string representation that can
    /// be sent over the message broker.
    ///
    /// # Returns
    ///
    /// Returns a JSON string representation of the message.
    ///
    /// # Panics
    ///
    /// Panics if serialization fails (which should not happen for valid `Message` types).
    fn to_json(&self) -> String {
        self._to_json()
    }
}

/// Internal trait for message serialization
///
/// This trait is used internally to implement JSON serialization for `Message` types.
/// Users should not need to implement this trait directly.
pub trait MessageSerialize {
    /// Internal method to serialize to JSON
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

// Standard library types that implement Message
impl Message for () {}
impl Message for u128 {}
impl Message for String {}
impl Message for Vec<String> {}
