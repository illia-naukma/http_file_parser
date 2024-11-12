BINARY_NAME := http_file_parser
SRC := src
LIB := lib.rs
MAIN := main.rs

run:
	@cargo run -- $(ARGS)

build:
	@cargo build --release

test:
	@cargo test -- --nocapture

format:
	@cargo fmt

lint:
	@cargo clippy

pre-commit: format lint test
	@echo "All checks passed!"

clean:
	@cargo clean

doc:
	@cargo doc --open

release: pre-commit
	@cargo build --release

help:
	@echo "Makefile commands:"
	@echo "  run         - Run the program, e.g make run ARGS="parse --file src/test.http""
	@echo "  build       - Build the project"
	@echo "  test        - Run tests"
	@echo "  format      - Format the code"
	@echo "  lint        - Lint the code using Clippy"
	@echo "  pre-commit  - Run formatting, linting, and tests before committing"
	@echo "  clean       - Clean up build artifacts"
	@echo "  doc         - Generate and open documentation"
	@echo "  release     - Build the project in release mode with checks"
