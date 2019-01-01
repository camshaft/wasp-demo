watch:
	@cargo watch -x 'build --target wasm32-unknown-unknown --release'

build:
	@cargo build --target wasm32-unknown-unknown --release

run:
	@wasp run ./target/wasm32-unknown-unknown/release/app.wasm
