.PHONY: build test clean fmt

build:
	@echo "Building fusion-resolver contract..."
	cargo build --target wasm32-unknown-unknown --release
	@echo "Optimizing WASM..."
	@mkdir -p ../../target/wasm32-unknown-unknown/release/
	@cp ../../../target/wasm32-unknown-unknown/release/fusion_resolver.wasm ../../target/wasm32-unknown-unknown/release/

test:
	@echo "Running fusion-resolver tests..."
	cargo test

clean:
	@echo "Cleaning fusion-resolver..."
	cargo clean

fmt:
	@echo "Formatting fusion-resolver..."
	cargo fmt --all