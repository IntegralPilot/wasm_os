#ifndef NEW_HPP
#define NEW_HPP

#include "cstddef.hpp"  // Ensure this file is included to define size_t

namespace std {

// Standard library exception class for allocation failures
struct bad_alloc {
    const char* what() const noexcept {
        return "bad allocation";
    }
};

// nothrow_t
struct nothrow_t {};

} // namespace std

extern "C" {
    void* malloc(std::size_t size);
    void free(void* ptr);
}

// Custom global new operator
void* operator new(std::size_t size) {
    void* ptr = malloc(size);
    if (reinterpret_cast<int>(ptr) < 0) {
        throw std::bad_alloc();
    }
    return ptr;
}

// Custom global new[] operator
void* operator new[](std::size_t size) {
    void* ptr = malloc(size);
    if (reinterpret_cast<int>(ptr) < 0) {
        throw std::bad_alloc();
    }
    return ptr;
}

// void* operator new(std::size_t, void*) noexcept;
void* operator new(std::size_t size, void* ptr) noexcept {
    return ptr;
}

// Custom global new operator with nothrow
void* operator new(std::size_t size, const std::nothrow_t&) noexcept {
    void* ptr = malloc(size);
    if (reinterpret_cast<int>(ptr) < 0) {
        return nullptr;
    }
    return ptr;
}

// Custom global new[] operator with nothrow
void* operator new[](std::size_t size, const std::nothrow_t&) noexcept {
    void* ptr = malloc(size);
    if (reinterpret_cast<int>(ptr) < 0) {
        return nullptr;
    }
    return ptr;
}

// Custom global delete operator
void operator delete(void* ptr) noexcept {
    if (ptr) {
        free(ptr);
    }
}

// Custom global delete[] operator
void operator delete[](void* ptr) noexcept {
    if (ptr) {
        free(ptr);
    }
}

#endif // NEW_HPP
