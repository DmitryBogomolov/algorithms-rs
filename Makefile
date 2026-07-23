.PHONY: clean
clean:
	@cargo clean --workspace --verbose

.PHONY: build
build:
	@cargo build --all-targets --verbose

.PHONY: fmt
fmt:
	@cargo fmt --all --check

.PHONY: clippy
clippy:
	@cargo clippy --all-targets --all-features -- -D warnings

.PHONY: test
test:
	@cargo test --all --verbose $(FILTER)
