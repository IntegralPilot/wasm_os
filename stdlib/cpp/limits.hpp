#ifndef LIMITS_HPP
#define LIMITS_HPP

namespace std {

template <typename T>
class numeric_limits {
public:
    // Not all of these are required for the vector implementation, 
    // but they are part of the standard numeric_limits interface.

    // Integer limits
    static constexpr bool is_specialized = false;
    static constexpr T min() noexcept { return T(); }
    static constexpr T max() noexcept { return T(); }
    static constexpr int digits = 0;
    static constexpr int digits10 = 0;
    static constexpr int max_digits10 = 0; 

    // Floating-point limits (not used by vector, but provided for completeness)
    static constexpr bool is_signed = false; 
    static constexpr bool is_integer = false;
    static constexpr bool is_exact = false;
    static constexpr int radix = 0;
    static constexpr T epsilon() noexcept { return T(); } 
    static constexpr T round_error() noexcept { return T(); } 
    // ... [add other floating-point limits as needed] ...
};

// Specialization for size_t
template <>
class numeric_limits<size_t> {
public:
    static constexpr bool is_specialized = true;
    static constexpr size_t min() noexcept { return 0; }
    static constexpr size_t max() noexcept { return (size_t)-1; }
};

}  // namespace std

#endif // LIMITS_HPP