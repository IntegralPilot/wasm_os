CC = clang++
CFLAGS = -I ../../stdlib/cpp -nostdlib --target=wasm32 -Ofast
LDFLAGS = -Wl,--allow-undefined

all: main.wasm

main.wasm:
	copy ..\..\stdlib\glue\glue.cpp glue.cpp
	$(CC) $(CFLAGS) $(LDFLAGS) glue.cpp -o ./main.wasm
	move glue.cpp glue.cpp.generated

clean:
	del main.wasm
	del glue.c.generated
