// These functions are implemented in the kernel.
// State the signatures but don't define them so that clang will make the .wasm file dynamically link to them at runtime.
extern "C" int timesinceboot();
extern "C" int cputime();