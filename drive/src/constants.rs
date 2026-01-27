//! Constants used throughout the macro code generation
//!
//! This module defines string constants for trait and type paths used when
//! generating code in procedural macros.

/// Full path to the `Message` trait in the message_flow crate
pub const MESSAGE_TRAIT_PATH: &str = "message_flow::Message";

/// Full path to the `Register` trait in the message_flow crate
pub const REGISTER_TRAIT_PATH: &str = "message_flow::Register";

/// Simple name for the `Context` type
pub const CONTEXT_TYPE_NAME: &str = "Context";

/// Full path to the `Context` type in the message_flow crate
pub const CONTEXT_FULL_PATH: &str = "message_flow::Context";

