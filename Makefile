
expand:
	cargo expand --test code_gen > .tmp/expanded.rs

test: 
	cargo test --package message_flow --test code_gen -- --nocapture 