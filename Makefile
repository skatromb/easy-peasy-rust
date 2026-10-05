.PHONY: lint test build

lint:
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings

test:
	cargo test

build:
	cargo build --release
