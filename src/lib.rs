//! # message_flow
//!
//! A lightweight Rust library that provides an ergonomic and declarative way to build
//! services that communicate through message brokers (like NATS), using procedural macros.
//!
//! ## Overview
//!
//! `message_flow` abstracts message registration, pattern matching, and dispatching into
//! a clean macro-driven interface and supports async handlers. It enables you to build
//! type-safe, structured messaging systems with minimal boilerplate.
//!
//! ## Key Features
//!
//! - **Macro-driven API**: Use `#[msg_flow]` and `#[message]` attributes to define handlers
//! - **Async Support**: All handlers are async and work seamlessly with Tokio
//! - **Type Safety**: Leverages Rust's type system and `serde` for serialization
//! - **Broker Agnostic**: Core design supports multiple message brokers (currently NATS)
//!
//! ## Example
//!
//! ```rust,no_run
//! use message_flow::{msg_flow, message, registers, MsgDef};
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(MsgDef, Serialize, Deserialize, Debug)]
//! struct User {
//!     first_name: String,
//! }
//!
//! #[msg_flow(pattern = "service_A")]
//! impl User {
//!     #[message(pattern = "1")]
//!     async fn greeting(&self) -> Result<String, Box<dyn std::error::Error>> {
//!         Ok("Hello!".into())
//!     }
//! }
//! ```
//!
//! ## Modules
//!
//! - [`app`]: Application initialization and connection management
//! - [`context`]: Request context and error handling
//! - [`error`]: Error response types
//! - [`logger`]: Feature-gated logging macros
//! - [`util`]: Utility macros and functions
//!
//! See also: [`Register`], [`Message`], [`Context`], [`app::App`]

pub mod logger;

pub mod app;
pub mod error;
pub mod context;
pub mod result;
pub mod util;
mod handler;
mod message;

/// Re-exported context type for message handlers
pub use context::Context;

/// Default error response schema
pub use error::DefaultResponseErrorSchema;

/// Trait for registering message handlers with the broker
pub use handler::Register;

/// Trait for types that can be serialized as messages
pub use message::Message;

/// Result type alias for NATS operations
pub use result::Result;

/// Utility macros and functions
pub use util::*;

/// Re-exported NATS client type
pub use async_nats::Client;

/// Re-exported async trait for trait definitions
pub use async_trait::async_trait;

/// Re-exported bytes type for message payloads
pub use bytes::Bytes;

// Re-export procedural macros from message_flow_drive
pub use message_flow_drive::{msg_flow, MsgDef};
