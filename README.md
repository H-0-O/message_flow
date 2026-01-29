# 📨 message_flow

`message_flow` is a lightweight Rust library that provides an ergonomic and declarative way to build services that communicate through message brokers (like NATS), using procedural macros. It abstracts message registration, pattern matching, and dispatching into a clean macro-driven interface and supports async handlers.

## ✨ Features

- ✅ Easy-to-use macro system for message definition  
- ⚡ Asynchronous message handling  
- 🔁 Simple registration of multiple services  
- 🔒 Type-safe and structured messaging using `serde`  
- 🌐 Broker-agnostic core (starting with NATS support)

---

## 🚀 Getting Started

### 📄 Define Message Handlers

```rust
use message_flow::{msg_flow, registers, MsgDef, Result};
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

> Note: `#[message(...)]` is a **helper attribute** that is consumed by `#[msg_flow]`.
> You do not import a `message` proc-macro — just use the `#[message]` attribute on methods
> inside a `#[msg_flow]` impl block.

---

## 🔌 Connecting to the Broker

```rust
use message_flow::{App, registers};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new("localhost:4222".to_string()).await?;
    app.connect_and_wait(registers!(User, User2)).await?;
    Ok(())
}
```

### Using RabbitMQ instead of NATS

Enable the `rabbitmq` feature and disable `nats` in your `Cargo.toml`:

```toml
[dependencies]
message_flow = { path = ".", default-features = false, features = ["rabbitmq"] }
```

Then pass an AMQP URI to `App::new`:

```rust
use message_flow::{App, registers};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new("amqp://guest:guest@localhost:5672/%2f".to_string()).await?;
    app.connect_and_wait(registers!(User, User2)).await?;
    Ok(())
}
```

### Custom Error Handling

You can customize how errors are handled and formatted:

```rust
use message_flow::{App, DefaultResponseErrorSchema, registers};

let app = App::new("localhost:4222".to_string()).await?
    .set_error_handler(|code, msg, _client| {
        DefaultResponseErrorSchema {
            code,
            message: format!("Custom error: {}", msg),
        }
    });
app.connect_and_wait(registers!(User)).await?;
```

---

## 🧠 Macro Reference

### `#[derive(MsgDef)]`

Derives the `Message` trait for a struct, allowing it to be used as a message type.

**Requirements:**

- Must also derive `Serialize`, `Deserialize` from `serde`
- Must derive `Debug`

**Example:**

```rust
#[derive(MsgDef, Serialize, Deserialize, Debug)]
struct MyMessage {
    data: String,
}
```

### `#[msg_flow(pattern = "...")]`

Marks an `impl` block as containing message handlers. The optional `pattern` parameter
defines a base topic pattern that is prepended to all handler patterns.

**Arguments:**

- `pattern` (optional): Base pattern/topic for all handlers in this impl block

**Example:**

```rust
#[msg_flow(pattern = "service_name")]
impl MyHandler {
    // handlers here
}
```

### `#[message(pattern = "...")]`

Marks a method as a message handler. The method must be async and return a `Result`.

**Arguments:**

- `pattern` (required): The message pattern/topic for this handler

**Handler Signature:**

- Must be an async method
- Can optionally take `&Context` as a parameter
- Must return `message_flow::Result<T>` where `T: Message` (i.e. `Result<T, async_nats::Error>`)

**Example:**

```rust
#[message(pattern = "handler1")]
async fn handle(&self) -> message_flow::Result<String> {
    Ok("Response".into())
}

// With context
#[message(pattern = "handler2")]
async fn handle_with_context(&self, context: &Context) -> message_flow::Result<String> {
    // Use context.client to send messages
    Ok("Response".into())
}
```

### `registers!(...)`

Macro that creates an array of registration functions for multiple handler types.

**Arguments:**

- One or more handler types (comma-separated, trailing comma allowed)

**Example:**

```rust
registers!(User, User2, MyHandler)
```

---

## 📚 Architecture Overview

### Core Components

1. **Message Types**: Structs that derive `MsgDef` and implement `Serialize`/`Deserialize`
2. **Handlers**: Methods annotated with `#[message]` that process incoming messages
3. **Registration**: The `Register` trait (auto-implemented) that sets up broker subscriptions
4. **Application**: The `App` struct that manages broker connections and handler registration

### Message Flow

```text
1. Define Message Struct → #[derive(MsgDef)]
2. Implement Handlers → #[msg_flow] + #[message]
3. Register with App → App::connect_and_wait(registers!(...))
4. Broker receives message → Deserializes to struct
5. Handler invoked → Returns response
6. Response sent back → Serialized and published
```

### Pattern Matching

Message patterns are constructed by combining:

- Base pattern from `#[msg_flow(pattern = "...")]` (optional)
- Handler pattern from `#[message(pattern = "...")]`

**Example:**

- `#[msg_flow(pattern = "service")]` + `#[message(pattern = "handler")]`
- Results in subscription to: `"service.handler"`

---

## 🧩 Dynamic Schema (raw JSON handlers)

Sometimes you want to accept **arbitrary JSON** (no strongly-typed request struct). `message_flow`
supports this via a *dynamic schema* handler.

### How it works

Inside a `#[msg_flow]` impl block, a handler must be **either**:

- **Static schema**: `async fn handler(&self, ...optional &Context...) -> message_flow::Result<T>`
  - Payload is deserialized into `Self` (`serde_json::from_slice::<Self>(...)`)
- **Dynamic schema**: `async fn handler(input: serde_json::Value, ...optional &Context...) -> message_flow::Result<T>`
  - Payload is deserialized into `serde_json::Value` and passed to an **associated function**
  - The handler **must not** take `&self`

If you try to mix `&self` and `serde_json::Value` (or use neither), the macro emits a compile error.

### Example

```rust
use message_flow::{msg_flow, MsgDef, Result};
use serde::{Deserialize, Serialize};

#[derive(MsgDef, Debug, Serialize, Deserialize)]
struct Dynamic;

#[derive(MsgDef, Debug, Serialize, Deserialize)]
struct EchoResponse {
    message: String,
}

#[msg_flow(pattern = "dynamic")]
impl Dynamic {
    // Dynamic schema handler: no `&self`, takes raw JSON payload.
    #[message(pattern = "echo")]
    async fn echo(input: serde_json::Value) -> Result<EchoResponse> {
        Ok(EchoResponse {
            message: format!("got: {}", input),
        })
    }
}
```

Send a request (subject is `base_pattern + "." + message_pattern` → `dynamic.echo`):

```bash
nats request dynamic.echo '{"any":"shape","works":true}'
```

### Error Handling

- **Handler Errors**: Caught and passed to the error handler
- **Deserialization Errors**: Return 422 status with error message
- **Custom Error Handler**: Set via `App::set_error_handler()`

---

## 🧪 Testing

### Prerequisites

Start NATS server locally:

```bash
nats-server
```

### Running Tests

```bash
cargo test
```

If you want to run the integration test that requires NATS, run ignored tests:

```bash
cargo test -- --ignored
```

### Testing with NATS CLI

Send messages from the command line:

```bash
# Request-reply pattern (expects response)
nats request service_A.1 '{"first_name":"John"}' 

# Publish message
nats pub service_A.1 '{"first_name":"John"}'
```

### Example Test

```rust
use message_flow::{App, registers, msg_flow, MsgDef, Result};
use serde::{Serialize, Deserialize};

#[derive(MsgDef, Serialize, Deserialize, Debug)]
struct TestMessage {
    value: i32,
}

#[msg_flow(pattern = "test")]
impl TestMessage {
    #[message(pattern = "echo")]
    async fn echo(&self) -> Result<i32> {
        Ok(self.value * 2)
    }
}

#[tokio::test]
async fn test_handler() {
    let app = App::new("localhost:4222".to_string()).await.unwrap();
    // Test your handlers
}
```

---

## 📖 API Documentation

### Core Types

- **`App`**: Main application struct for managing connections
- **`Context`**: Request context with broker client and error handler
- **`Message`**: Trait for serializable message types
- **`Register`**: Trait for handler registration (auto-implemented)
- **`Result<T>`**: Type alias for `Result<T, async_nats::Error>`

### Key Functions

- **`App::new(addr)`**: Create a new app and connect to broker
- **`App::connect(registers)`**: Register handlers and start listening
- **`App::connect_and_wait(registers)`**: Connect and wait for shutdown signal
- **`App::set_error_handler(callback)`**: Set custom error handler

### Logging

Feature-gated logging macros are available:

```rust
use message_flow::{debug_log, info_log, warn_log, error_log, trace_log};

info_log!("Application started");
debug_log!("Debug information: {}", value);
```

Enable logging features in `Cargo.toml`:

```toml
[features]
default = ["nats", "debug", "info", "warn", "error", "trace"]
```

The logging macros are compiled only when **both** `log` and the level feature are enabled
(e.g. `log` + `info`). By default, this crate enables the level flags but does **not** enable
`log`, so enable it in your dependency if you want logs:

```toml
message_flow = { path = ".", features = ["log"] }
```

## 📦 Planned Features

- 📢 `#[event]` macro for fire-and-forget event broadcasting  
- 🔌 Middleware support  
- 🔒 Authentication hooks  
- 📊 Telemetry / metrics support  
- 🌍 Pluggable message backends

For detailed roadmap and development plans, see [ROADMAP.md](./ROADMAP.md)  
For immediate tasks and bugs, see [TODO.md](./TODO.md)

## 🔍 Project Structure

```text
message_flow/
├── src/              # Main library code
│   ├── app.rs        # Application initialization
│   ├── context.rs    # Request context
│   ├── error.rs      # Error types
│   ├── handler.rs    # Register trait
│   ├── message.rs    # Message trait
│   ├── logger.rs     # Logging macros
│   └── util.rs       # Utility macros
├── drive/            # Procedural macros
│   └── src/
│       ├── attribute.rs  # Attribute parsing
│       ├── handler.rs    # Handler analysis
│       ├── msg_def.rs   # MsgDef derive
│       ├── msg_flow.rs  # msg_flow attribute
│       └── register.rs  # Registration codegen
└── tests/            # Integration tests
```

---

## 📄 License

Licensed under the [MIT License](LICENSE).

---

## 🤝 Contributing

Contributions, ideas, and bug reports are welcome!

---

## 👋 Author

Crafted with ❤️ in Rust.
