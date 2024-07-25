default: main.wasm

main.wasm:
	cargo build --release --target wasm32-unknown-unknown
	copy "target\wasm32-unknown-unknown\release\helloworld_rs.wasm" "main.wasm"

clean:
	del main.wasm
	cargo clean
