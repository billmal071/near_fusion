.PHONY: build test clean fmt

build:
	@echo "Building escrow-src contract..."
	cargo build --target wasm32-unknown-unknown --release
	@echo "Optimizing WASM..."
	@mkdir -p ../../target/wasm32-unknown-unknown/release/
	@cp ../../../target/wasm32-unknown-unknown/release/escrow_src.wasm ../../target/wasm32-unknown-unknown/release/

test:
	@echo "Running escrow-src tests..."
	cargo test

clean:
	@echo "Cleaning escrow-src..."
	cargo clean

fmt:
	@echo "Formatting escrow-src..."
	cargo fmt --all