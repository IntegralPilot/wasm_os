#ifndef _UTILITY_HPP
#define _UTILITY_HPP

#include <type_traits.hpp>

// Custom memmove
void* memmove(void* dest, const void* src, std::size_t n) {
    char* csrc = (char*)src;
    char* cdest = (char*)dest;
    if (csrc < cdest) {
        for (std::size_t i = n; i > 0; i--) {
            cdest[i - 1] = csrc[i - 1];
        }
    } else {
        for (std::size_t i = 0; i < n; i++) {
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

#endif
