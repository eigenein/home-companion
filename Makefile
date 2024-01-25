.PHONY: release
release:
	@cargo build --release --package=home-companion
	@cargo build --release --target=wasm32-unknown-unknown --package=connection-youless

.PHONY: debug
debug:
	@cargo build --package=home-companion
	@cargo build --target=wasm32-unknown-unknown --package=connection-youless

.PHONY: fmt format
fmt format:
	@cargo +nightly fmt --all

.PHONY: udeps
	@cargo +nightly udeps --workspace
