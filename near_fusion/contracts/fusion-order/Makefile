.PHONY: build test clean fmt

build:
	@echo "Building fusion-order contract..."
	cargo build --target wasm32-unknown-unknown --release
	@echo "Optimizing WASM..."
	@mkdir -p ../../target/wasm32-unknown-unknown/release/
	@cp ../../../target/wasm32-unknown-unknown/release/fusion_order.wasm ../../target/wasm32-unknown-unknown/release/

test:
	@echo "Running fusion-order tests..."
	cargo test

clean:
	@echo "Cleaning fusion-order..."
	cargo clean

fmt:
	@echo "Formatting fusion-order..."
	cargo fmt --all