#include <stddef.h>

void* malloc(size_t size);
void free(void* ptr);
int ptrsize(void* ptr);

// make our own memcpy function
void* memcpy(void* dest, const void* src, size_t n) {
    char* csrc = (char*)src;
    char* cdest = (char*)dest;
    for (size_t i = 0; i < n; i++) {
        cdest[i] = csrc[i];
    }
    return dest;
}

// make our own realloc function
void* realloc(void* ptr, size_t new_size) {
    if (ptr == NULL) {
        return malloc(new_size);
    }

    // You need to keep track of the old size to properly copy the data
    int old_size = ptrsize(ptr);
    
    void* new_ptr = malloc(new_size);
    if (new_ptr == NULL) {
        return NULL; // Allocation failed
    }

    // Copy the old data to the new location
    memcpy(new_ptr, ptr, old_size < new_size ? old_size : new_size);

    // Free the old location
    free(ptr);

    return new_ptr;
}

// A simple implementation of atoi
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