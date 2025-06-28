default: main.wasm

main.wasm: node_modules
	npm run asbuild:release
	cp ./build/release.wasm main.wasm

node_modules:
	npm i

clean:
	rm -r -fo node_modules	
	rm -r -fo build