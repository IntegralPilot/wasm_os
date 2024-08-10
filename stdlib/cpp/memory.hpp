#ifndef MEMORY_HPP
#define MEMORY_HPP

#include "type_traits.hpp" // for std::remove_reference

#include <cstddef.hpp>  // for unsigned long
#include <new.hpp>      // for std::badalloc

extern "C" {
    void* malloc(unsigned long size);
    void free(void* ptr);
    int ptrsize(void* ptr);
}

// Custom global new operator
void* operator new(unsigned long size) {
    void* ptr = malloc(size);
    if (!ptr) {
        throw std::bad_alloc();
    }
    return ptr;
}

// Custom global new[] operator
void* operator new[](unsigned long size) {
    void* ptr = malloc(size);
    if (!ptr) {
        throw std::bad_alloc();
    }
    return ptr;
}

// Custom global new operator with nothrow
void* operator new(unsigned long size, const std::nothrow_t&) noexcept {
    return malloc(size);
}

// Custom global new[] operator with nothrow
void* operator new[](unsigned long size, const std::nothrow_t&) noexcept {
    return malloc(size);
}

// Custom global delete operator
void operator delete(void* ptr) noexcept {
    free(ptr);
}

// Custom global delete[] operator
void operator delete[](void* ptr) noexcept {
    free(ptr);
}

// Custom global delete operator with size
void operator delete(void* ptr, unsigned long size) noexcept {
    free(ptr);
}

// Custom global delete[] operator with size
void operator delete[](void* ptr, unsigned long size) noexcept {
    free(ptr);
}

// make memmove
void* memmove(void* dest, const void* src, unsigned long n) {
    char* csrc = (char*)src;
    char* cdest = (char*)dest;
    if (csrc < cdest) {
        for (unsigned long i = n; i > 0; i--) {
            cdest[i - 1] = csrc[i - 1];
        }
    } else {
        for (unsigned long i = 0; i < n; i++) {
            cdest[i] = csrc[i];
        }
    }
    return dest;
}

namespace std {
    // make std::move using memmove (3 arguments, specific use case)
    template <typename T>
    T* move(T* dest, const T* src, unsigned long n) {
        return static_cast<T*>(memmove(dest, src, n * sizeof(T)));
    }

    // make std::move (standard 1-argument version)
    template<typename T>
    constexpr typename std::remove_reference<T>::type&& move(T&& t) noexcept {
        return static_cast<typename std::remove_reference<T>::type&&>(t);
    }

    // make std::forward 
    template <typename T>
    T&& forward(T& t) noexcept {
        return static_cast<T&&>(t);
    }
}

#endif // MEMORY_HPP
