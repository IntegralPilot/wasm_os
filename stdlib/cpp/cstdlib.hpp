// These functions are implemented in the kernel.
// State the signatures but don't define them so that clang will make the .wasm file dynamically link to them at runtime.
extern "C" int runapp(const char* appname);

int system(const char* command) {
    return runapp(command);
}