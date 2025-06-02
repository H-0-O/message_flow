# 📨 message_flow

`message_flow` is a lightweight Rust library that provides an ergonomic and declarative way to build services that communicate through message brokers (like NATS), using procedural macros. It abstracts message registration, pattern matching, and dispatching into a clean macro-driven interface and supports async handlers.

## ✨ Features

- ✅ Easy-to-use macro system for message definition  
- ⚡ Asynchronous message handling  
- 🔁 Simple registration of multiple services  
- 🔒 Type-safe and structured messaging using `serde`  
- 🌐 Broker-agnostic core (starting with NATS support)

---

## 📦 Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
message_flow = "0.1.0" # Replace with your version
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
anyhow = "1"
```

---

## 🚀 Getting Started

### 📄 Define Message Handlers

```rust
use message_flow::{msg_flow, message, registers, MsgDef};
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(MsgDef, Serialize, Deserialize, Debug)]
struct User {
    first_name: String,
}

#[msg_flow(pattern = "service_A")]
impl User {
    #[message(pattern = "1")]
    async fn greeting(&self) -> Result<String> {
        println!("IN SERVICE 1");
        Ok("OK HELLO FROM GREETING".into())
    }

    #[message(pattern = "2")]
    async fn greeting2(&self) -> Result<String> {
        println!("IN SERVICE 2");
        Ok("OK HELLO FROM GREETING 2".into())
    }

    #[message(pattern = "3")]
    async fn greeting3(&self) -> Result<String> {
        println!("IN SERVICE 3");
        Ok("OK HELLO FROM GREETING 3".into())
    }
}

#[derive(MsgDef, Serialize, Deserialize, Debug)]
struct User2 {
    first_name: String,
}

#[msg_flow(pattern = "service_B")]
impl User2 {
    #[message(pattern = "ss")]
    async fn greeting(&self) -> Result<String> {
        println!("IN SERVICE B: greeting");
        Ok("OK HELLO FROM SERVICE B".into())
    }

    #[message(pattern = "service_2")]
    async fn ok(&self) -> Result<String> {
        println!("SERVICE B IS READY");
        Ok("OK HELLO FROM SERVICE B - OK".into())
    }
}
```

---

## 🔌 Connecting to the Broker

```rust
#[tokio::test]
async fn main() {
    println!("BEFORE");

    let _result = message_flow::connection::connect_and_wait(
        "localhost:4222".into(),
        registers!(User, User2)
    )
    .await
    .unwrap();

    let usr = User {
        first_name: "HO".into(),
    };

    // You can send or respond to messages
}
```

---

## 🧠 Macro Reference

- `#[msg_flow(pattern = "...")]`: Defines the root pattern/topic for message handlers.
- `#[message(pattern = "...")]`: Registers individual handler methods.
- `#[derive(MsgDef)]`: Derives traits required for a type to handle messages.
- `registers!(...)`: Combines multiple handler types for broker registration.

---

## 📚 Architecture Overview

```
Service Struct + #[msg_flow] → Base pattern
Handler Methods + #[message] → Sub-pattern routing
connect_and_wait → Connects to broker and binds handlers
```

---

## 🧪 Testing

Start NATS locally:

```bash
nats-server
```

Run and test:

```bash
cargo test
```

Send messages from CLI:

```bash
nats pub service_A.1 '{}'
nats pub service_B.ss '{}'
```

---

## 📦 Planned Features

- 📢 `#[event]` macro for fire-and-forget event broadcasting  
- 🔌 Middleware support  
- 🔒 Authentication hooks  
- 📊 Telemetry / metrics support  
- 🌍 Pluggable message backends

---

## 📄 License

Licensed under the [MIT License](LICENSE).

---

## 🤝 Contributing

Contributions, ideas, and bug reports are welcome!

---

## 👋 Author

Crafted with ❤️ in Rust.
