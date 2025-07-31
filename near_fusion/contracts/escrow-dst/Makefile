.PHONY: build test clean fmt

build:
	@echo "Building escrow-dst contract..."
	cargo build --target wasm32-unknown-unknown --release
	@echo "Optimizing WASM..."
	@mkdir -p ../../target/wasm32-unknown-unknown/release/
	@cp ../../../target/wasm32-unknown-unknown/release/escrow_dst.wasm ../../target/wasm32-unknown-unknown/release/

test:
	@echo "Running escrow-dst tests..."
	cargo test

clean:
	@echo "Cleaning escrow-dst..."
	cargo clean

fmt:
	@echo "Formatting escrow-dst..."
	cargo fmt --all