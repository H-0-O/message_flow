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
    /// Indicates whether this message type is a unit-like struct (no fields).
    ///
    /// - `true` for unit-like structs (e.g., `struct MyMessage;`)
    /// - `false` for data-bearing structs (e.g., `struct MyMessage { field: String }`)
    ///
    /// This constant is used to determine whether to deserialize the message into
    /// a struct or pass raw JSON as `serde_json::Value` to handlers.
    ///
    /// Note: This is a const associated item for compile-time evaluation.
    /// It cannot be accessed through trait objects, but can be used with
    /// `<Type as Message>::IS_UNIT` syntax.
    const IS_UNIT: bool;

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
impl Message for () {
    const IS_UNIT: bool = true;
}
impl Message for bool {
    const IS_UNIT: bool = false;
}
impl Message for i8 {
    const IS_UNIT: bool = false;
}
impl Message for i16 {
    const IS_UNIT: bool = false;
}
impl Message for i32 {
    const IS_UNIT: bool = false;
}
impl Message for i64 {
    const IS_UNIT: bool = false;
}
impl Message for i128 {
    const IS_UNIT: bool = false;
}
impl Message for isize {
    const IS_UNIT: bool = false;
}
impl Message for u8 {
    const IS_UNIT: bool = false;
}
impl Message for u16 {
    const IS_UNIT: bool = false;
}
impl Message for u32 {
    const IS_UNIT: bool = false;
}
impl Message for u64 {
    const IS_UNIT: bool = false;
}
impl Message for u128 {
    const IS_UNIT: bool = false;
}
impl Message for usize {
    const IS_UNIT: bool = false;
}
impl Message for f32 {
    const IS_UNIT: bool = false;
}
impl Message for f64 {
    const IS_UNIT: bool = false;
}
impl Message for char {
    const IS_UNIT: bool = false;
}
impl Message for String {
    const IS_UNIT: bool = false;
}
impl Message for std::collections::HashMap<String, String> {
    const IS_UNIT: bool = false;
}
impl Message for std::collections::BTreeMap<String, String> {
    const IS_UNIT: bool = false;
}
impl Message for std::collections::HashSet<String> {
    const IS_UNIT: bool = false;
}
impl Message for std::collections::BTreeSet<String> {
    const IS_UNIT: bool = false;
}
impl<T: Message + Serialize + DeserializeOwned> Message for Option<T> {
    const IS_UNIT: bool = false;
}
impl<T: Message + Serialize + DeserializeOwned> Message for Vec<T> {
    const IS_UNIT: bool = false;
}
impl<T1: Message + Serialize + DeserializeOwned, T2: Message + Serialize + DeserializeOwned> Message for (T1, T2) {
    const IS_UNIT: bool = false;
}
impl<T1: Message + Serialize + DeserializeOwned, T2: Message + Serialize + DeserializeOwned, T3: Message + Serialize + DeserializeOwned> Message for (T1, T2, T3) {
    const IS_UNIT: bool = false;
}
impl<T1: Message + Serialize + DeserializeOwned, T2: Message + Serialize + DeserializeOwned, T3: Message + Serialize + DeserializeOwned, T4: Message + Serialize + DeserializeOwned> Message for (T1, T2, T3, T4) {
    const IS_UNIT: bool = false;
}
impl<T1: Message + Serialize + DeserializeOwned, T2: Message + Serialize + DeserializeOwned, T3: Message + Serialize + DeserializeOwned, T4: Message + Serialize + DeserializeOwned, T5: Message + Serialize + DeserializeOwned> Message for (T1, T2, T3, T4, T5) {
    const IS_UNIT: bool = false;
}
