CC = clang
CFLAGS = -I ../../stdlib/c -nostdlib --target=wasm32 -Ofast
LDFLAGS = -Wl,--allow-undefined

all: main.wasm

main.wasm:
	cp ../../stdlib/glue/glue.c glue.c
	$(CC) $(CFLAGS) $(LDFLAGS) glue.c -o ./main.wasm
	mv glue.c glue.c.generated

clean:
	rm main.wasm
	rm glue.c.generated
