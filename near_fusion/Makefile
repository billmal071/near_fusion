.PHONY: build test clean fmt build-all test-all clean-all fmt-all

CONTRACTS := escrow-src escrow-dst escrow-factory fusion-order fusion-resolver

build-all:
	@echo "Building all NEAR contracts..."
	@for contract in $(CONTRACTS); do \
		echo "Building $$contract..."; \
		$(MAKE) -C contracts/$$contract build; \
	done

test-all:
	@echo "Testing all NEAR contracts..."
	cargo test --workspace

clean-all:
	@echo "Cleaning all NEAR contracts..."
	cargo clean
	@for contract in $(CONTRACTS); do \
		$(MAKE) -C contracts/$$contract clean; \
	done

fmt-all:
	@echo "Formatting all NEAR contracts..."
	cargo fmt --all

build:
	$(MAKE) build-all

test:
	$(MAKE) test-all

clean:
	$(MAKE) clean-all

fmt:
	$(MAKE) fmt-all