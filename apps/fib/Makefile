CC = clang
CFLAGS = -I ../../stdlib/c -nostdlib --target=wasm32 -Ofast
LDFLAGS = -Wl,--allow-undefined

all: main.wasm

main.wasm:
	copy ..\..\stdlib\glue\glue.c glue.c
	$(CC) $(CFLAGS) $(LDFLAGS) glue.c -o ./main.wasm
	move glue.c glue.c.generated

clean:
	del main.wasm
	del glue.c.generated
