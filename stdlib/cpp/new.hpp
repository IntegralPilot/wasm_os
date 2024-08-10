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

#endif // NEW_HPP