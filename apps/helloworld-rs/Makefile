default: main.wasm

main.wasm:
	cargo build --release --target wasm32-unknown-unknown
	cp "target/wasm32-unknown-unknown/release/helloworld_rs.wasm" "main.wasm"

clean:
	rm main.wasm
	cargo clean
