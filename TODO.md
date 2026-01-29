# 📝 Immediate TODO List

Quick reference for immediate tasks and bugs to fix.

## 🐛 Critical Bugs

- [x] Fix unused variable `addr` in `src/app.rs:27` (no longer applicable; `addr` is used)
- [ ] Replace panic in `src/message.rs:21` with proper error handling
- [ ] Add input validation in macro code (previous reference to `drive/src/msg_flow.rs:178` appears outdated; add a fresh concrete location/issue)

## 🚀 High Priority Features

- [ ] Complete `#[event]` macro implementation (partially done, needs testing)
- [ ] Create custom error types (replace `Box<dyn std::error::Error>`)
- [ ] Improve error messages and context

## 📚 Documentation

- [ ] Add event macro examples to README
- [x] Document dynamic schema vs static schema (added to README)
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

