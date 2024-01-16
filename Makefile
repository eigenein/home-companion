.PHONY: build/release
build/release:
	@cargo build --release --package=home-companion
	@cargo build --release --target=wasm32-unknown-unknown --package=plugin-youless

.PHONY: build/debug
build/debug:
	@cargo build --package=home-companion
	@cargo build --target=wasm32-unknown-unknown --package=plugin-youless
