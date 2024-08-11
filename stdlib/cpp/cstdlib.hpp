// These functions are implemented in the kernel.
// State the signatures but don't define them so that clang will make the .wasm file dynamically link to them at runtime.
extern "C" {
    int runapp(const char* appname);
    int seedrng(long int* seed);
    int rng();
}

namespace std {

int system(const char* command) {
    return runapp(command);
}

void srand(unsigned int seed) {
    seedrng((long int*)&seed);
}

int rand() {
    return rng();
}

int atoi(const char* str) {
    // first check that there are not any non-digit characters
    for (int i = 0; str[i] != '\0'; i++) {
        if (str[i] < '0' || str[i] > '9') {
            return 0;
        }
    }
    int res = 0;
    int sign = 1;
    int i = 0;

    if (str[0] == '-') {
        sign = -1;
        i++;
    }

    for (; str[i] != '\0'; ++i) {
        res = res * 10 + str[i] - '0';
    }

    return sign * res;
}

} // namespace std