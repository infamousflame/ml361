# Makefile for ml361

.PHONY: build test format clean

# Build the Rust library and install it in the local Python environment
build:
	maturin develop

# Run both Rust and Python tests
test: build
	cargo test
	pytest

# Format the code with an 80-character limit
format:
	rustfmt --config max_width=80 *.rs
	ruff format --line-length 80 *.py

# Clean build artifacts
clean:
	cargo clean
	rm -rf target/
