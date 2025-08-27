.PHONY: expand test test-simple

# default test name if none given
name ?= code_gen

expand:
	mkdir -p .tmp
	cargo expand --features "info" --test $(name) > .tmp/expanded.rs

test:
	cargo test --features "info debug" --package message_flow --test $(name) -- --nocapture

test-simple:
	cargo test

