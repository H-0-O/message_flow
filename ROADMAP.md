# 🗺️ message_flow Roadmap

This document outlines the planned features, improvements, and milestones for the `message_flow` project.

## 📋 Table of Contents

- [Current Status](#current-status)
- [Short-term Goals (v0.2.0)](#short-term-goals-v020)
- [Medium-term Goals (v0.3.0)](#medium-term-goals-v030)
- [Long-term Goals (v0.4.0+)](#long-term-goals-v040)
- [Code Quality & Maintenance](#code-quality--maintenance)
- [Documentation](#documentation)
- [Testing](#testing)

---

## Current Status

**Version:** 0.1.0  
**Status:** Early development, core functionality working

### ✅ Completed Features
- Basic message handler registration with `#[msg_flow]` and `#[message]` macros
- NATS broker integration
- Async message handling
- Type-safe message serialization/deserialization with serde
- Error handler support
- Context passing to handlers
- Basic logging infrastructure

### 🔄 Partially Implemented
- Event support (code exists but needs completion and testing)

---

## Short-term Goals (v0.2.0)

### 🎯 Core Features

#### 1. Complete Event Broadcasting (`#[event]` macro)
- [ ] Finalize event pattern implementation in `msg_flow.rs`
- [ ] Add fire-and-forget event broadcasting (no reply expected)
- [ ] Update documentation with event examples
- [ ] Add tests for event handling
- [ ] Ensure events work correctly with NATS pub/sub

**Priority:** High  
**Estimated Effort:** 2-3 days

#### 2. Improve Error Handling
- [ ] Replace panic in `message.rs` (line 21) with proper error handling
- [ ] Create custom error types instead of using `Box<dyn std::error::Error>`
- [ ] Add structured error responses with error codes
- [ ] Improve deserialization error messages
- [ ] Add error context/tracing

**Priority:** High  
**Estimated Effort:** 3-4 days

#### 3. Message Validation
- [ ] Add validation hooks for incoming messages
- [ ] Support custom validation functions
- [ ] Add built-in validators (e.g., required fields, format checks)

**Priority:** Medium  
**Estimated Effort:** 2-3 days

#### 4. Better Logging
- [ ] Integrate with popular logging crates (tracing, log, env_logger)
- [ ] Add structured logging with context
- [ ] Configurable log levels per component
- [ ] Request/response logging middleware

**Priority:** Medium  
**Estimated Effort:** 2 days

---

## Medium-term Goals (v0.3.0)

### 🔌 Middleware Support

#### 5. Middleware System
- [ ] Design middleware trait/interface
- [ ] Implement request/response middleware pipeline
- [ ] Add built-in middleware:
  - [ ] Request logging
  - [ ] Rate limiting
  - [ ] Request timeout
  - [ ] Request ID generation
- [ ] Support custom middleware
- [ ] Middleware ordering and composition

**Priority:** High  
**Estimated Effort:** 5-7 days

#### 6. Dynamic Schema Support
- [ ] Support "dynamic schema" message types using empty marker structs with `#[derive(MsgDef)]` (e.g. `struct Dynamic;`)
- [ ] Allow `#[msg_flow]` impl blocks on these marker types with non‑`self` handlers (no parsed input), forwarding the raw JSON payload from the client
- [ ] Enforce at compile time that dynamic handlers return `Result<..>` (or adapt generated code accordingly), and emit clear errors otherwise
- [ ] Document dynamic vs static schemas and provide examples (e.g. `Dynamic.greeting` that receives raw JSON)

**Priority:** Medium  
**Estimated Effort:** 3-4 days

#### 7. Authentication & Authorization
- [ ] Design auth hook system
- [ ] Support JWT validation
- [ ] API key authentication
- [ ] Custom auth providers
- [ ] Role-based access control (RBAC) support
- [ ] Context injection for authenticated user info

**Priority:** High  
**Estimated Effort:** 5-6 days

#### 8. Request/Response Interceptors
- [ ] Pre-handler interceptors
- [ ] Post-handler interceptors
- [ ] Error interceptors
- [ ] Response transformation

**Priority:** Medium  
**Estimated Effort:** 3-4 days

### 📊 Observability

#### 8. Telemetry & Metrics
- [ ] Integration with Prometheus/metrics crate
- [ ] Request count metrics
- [ ] Latency/histogram metrics
- [ ] Error rate metrics
- [ ] Custom metric support
- [ ] Distributed tracing support (OpenTelemetry)

**Priority:** Medium  
**Estimated Effort:** 4-5 days

#### 9. Health Checks
- [ ] Built-in health check endpoint/handler
- [ ] Readiness probes
- [ ] Liveness probes
- [ ] Custom health check hooks

**Priority:** Low  
**Estimated Effort:** 2 days

---

## Long-term Goals (v0.4.0+)

### 🌍 Broker Abstraction

#### 10. Pluggable Message Backends
- [ ] Design broker abstraction trait
- [ ] Implement RabbitMQ backend
- [ ] Implement Redis Streams backend
- [ ] Implement Kafka backend (if feasible)
- [ ] Backend feature flags
- [ ] Backend-specific configuration

**Priority:** Medium  
**Estimated Effort:** 10-15 days

#### 11. Message Queue Patterns
- [ ] Request-reply pattern (current)
- [ ] Pub/sub pattern
- [ ] Work queue pattern
- [ ] Fan-out pattern
- [ ] Topic routing

**Priority:** Medium  
**Estimated Effort:** 5-7 days

### 🚀 Performance & Scalability

#### 12. Connection Pooling
- [ ] Connection pool management
- [ ] Configurable pool sizes
- [ ] Connection health monitoring
- [ ] Automatic reconnection

**Priority:** Medium  
**Estimated Effort:** 3-4 days

#### 13. Message Batching
- [ ] Batch message processing
- [ ] Configurable batch sizes
- [ ] Batch timeout handling

**Priority:** Low  
**Estimated Effort:** 3-4 days

### 🔧 Developer Experience

#### 14. Code Generation Tools
- [ ] CLI tool for scaffolding message handlers
- [ ] Code generation from OpenAPI/schema definitions
- [ ] Message type generator

**Priority:** Low  
**Estimated Effort:** 5-7 days

#### 15. Configuration Management
- [ ] Configuration file support (YAML/TOML)
- [ ] Environment variable support
- [ ] Configuration validation
- [ ] Hot-reload configuration (optional)

**Priority:** Medium  
**Estimated Effort:** 3-4 days

---

## Code Quality & Maintenance

### 🐛 Bug Fixes & Improvements

- [ ] Fix unused variable warning in `app.rs` (line 27: `addr`)
- [ ] Review and improve error messages throughout codebase
- [ ] Add input validation in macro code (see TODO in `msg_flow.rs` line 178)
- [ ] Improve type safety in message handling
- [ ] Add bounds checking for patterns
- [ ] Handle edge cases in message deserialization

**Priority:** High  
**Estimated Effort:** Ongoing

### 🏗️ Architecture Improvements

- [ ] Refactor error handling to use `thiserror` or `anyhow` consistently
- [ ] Improve separation of concerns
- [ ] Add trait bounds documentation
- [ ] Consider using `async-trait` more consistently
- [ ] Review and optimize macro code generation

**Priority:** Medium  
**Estimated Effort:** Ongoing

### 📦 Dependencies

- [ ] Audit dependencies for security vulnerabilities
- [ ] Update dependencies to latest stable versions
- [ ] Consider reducing dependency footprint
- [ ] Document dependency choices

**Priority:** Medium  
**Estimated Effort:** 1-2 days

---

## Documentation

### 📚 Documentation Improvements

- [ ] API documentation with examples
- [ ] Architecture deep-dive document
- [ ] Migration guide for future breaking changes
- [ ] Performance best practices guide
- [ ] Troubleshooting guide
- [ ] FAQ section
- [ ] Contributing guidelines
- [ ] Code of conduct

**Priority:** High  
**Estimated Effort:** 5-7 days

### 📖 Examples & Tutorials

- [ ] Complete working examples for all features
- [ ] Tutorial: Building a microservice with message_flow
- [ ] Tutorial: Adding middleware
- [ ] Tutorial: Authentication setup
- [ ] Example: Multi-service communication
- [ ] Example: Error handling patterns

**Priority:** Medium  
**Estimated Effort:** 4-5 days

---

## Testing

### ✅ Test Coverage

- [ ] Unit tests for core functionality
- [ ] Integration tests with NATS
- [ ] Macro code generation tests
- [ ] Error handling tests
- [ ] Middleware tests (when implemented)
- [ ] Authentication tests (when implemented)
- [ ] Performance/benchmark tests
- [ ] Property-based tests for message serialization

**Priority:** High  
**Estimated Effort:** 7-10 days

### 🧪 Test Infrastructure

- [ ] Test utilities and helpers
- [ ] Mock broker for unit testing
- [ ] Test fixtures
- [ ] CI/CD pipeline setup
- [ ] Code coverage reporting

**Priority:** Medium  
**Estimated Effort:** 3-4 days

---

## 🎯 Milestones

### Milestone 1: v0.2.0 (Target: ~3-4 weeks)
- Complete event support
- Improved error handling
- Better logging
- Basic validation

### Milestone 2: v0.3.0 (Target: ~8-10 weeks)
- Middleware system
- Authentication hooks
- Telemetry support
- Comprehensive tests

### Milestone 3: v0.4.0 (Target: ~16-20 weeks)
- Broker abstraction
- Additional backends
- Performance improvements
- Production-ready features

---

## 🤝 Contributing

We welcome contributions! If you'd like to work on any of these items:

1. Check if there's an existing issue or create one
2. Comment on the issue to claim it
3. Submit a PR with your changes
4. Ensure tests pass and documentation is updated

---

## 📝 Notes

- Priorities are subject to change based on user feedback
- Estimated effort is approximate and may vary
- Some features may be combined or split based on implementation complexity
- Community feedback will help shape the roadmap

---

**Last Updated:** 2024  
**Maintainer:** [Your Name/Team]
