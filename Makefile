build:
	cargo build
format:
	cargo clean
	cargo fmt
	cargo clippy
run:
	cargo run --release
