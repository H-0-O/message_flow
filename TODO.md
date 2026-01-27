# 📝 Immediate TODO List

Quick reference for immediate tasks and bugs to fix.

## 🐛 Critical Bugs

- [ ] Fix unused variable `addr` in `src/app.rs:27` (used in logger but not in scope)
- [ ] Replace panic in `src/message.rs:21` with proper error handling
- [ ] Add input validation in macro code (`drive/src/msg_flow.rs:178` - see TODO comment)

## 🚀 High Priority Features

- [ ] Complete `#[event]` macro implementation (partially done, needs testing)
- [ ] Create custom error types (replace `Box<dyn std::error::Error>`)
- [ ] Improve error messages and context

## 📚 Documentation

- [ ] Add event macro examples to README
- [ ] Document error handling patterns
- [ ] Add troubleshooting section

## 🧪 Testing

- [ ] Add tests for event handling
- [ ] Add error handling tests
- [ ] Improve test coverage

## 🔧 Code Quality

- [ ] Run `cargo clippy` and fix warnings
- [ ] Run `cargo fmt` to ensure consistent formatting
- [ ] Review and document all public APIs

---

**Note:** For the full roadmap with detailed milestones, see [ROADMAP.md](./ROADMAP.md)

