CC = clang++
CFLAGS = -I ../../stdlib/cpp -nostdlib --target=wasm32 -Ofast
LDFLAGS = -Wl,--allow-undefined

all: main.wasm

main.wasm:
	$(CC) $(CFLAGS) $(LDFLAGS) src/main.cpp -o ./main.wasm

clean:
	rm main.wasm
