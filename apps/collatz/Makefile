CC = clang++
CFLAGS = -I ../../stdlib/cpp -nostdlib --target=wasm32 -Ofast
LDFLAGS = -Wl,--allow-undefined

all: main.wasm

main.wasm:
	cp ../../stdlib/glue/glue.cpp glue.cpp
	$(CC) $(CFLAGS) $(LDFLAGS) glue.cpp -o ./main.wasm
	mv glue.cpp glue.cpp.generated

clean:
	rm main.wasm
	rm glue.c.generated
