.PHONY: build test clean fmt

build:
	@echo "Building escrow-factory contract..."
	cargo build --target wasm32-unknown-unknown --release
	@echo "Optimizing WASM..."
	@mkdir -p ../../target/wasm32-unknown-unknown/release/
	@cp ../../../target/wasm32-unknown-unknown/release/escrow_factory.wasm ../../target/wasm32-unknown-unknown/release/

test:
	@echo "Running escrow-factory tests..."
	cargo test

clean:
	@echo "Cleaning escrow-factory..."
	cargo clean

fmt:
	@echo "Formatting escrow-factory..."
	cargo fmt --all