BINARY = shopping-parser

.PHONY: all build run test format clippy clean

build:
	cargo build

run:
	cargo run -- --help

test:
	cargo test

format:
	cargo fmt

clippy:
	cargo clippy

precommit: format clippy test

clean:
	cargo clean
