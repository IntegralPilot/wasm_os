#ifndef TYPE_TRAITS_HPP
#define TYPE_TRAITS_HPP

namespace std {

    // Primary template (for non-reference types)
    template<typename T>
    struct remove_reference {
        typedef T type;
    };

    // Specialization for lvalue reference
    template<typename T>
    struct remove_reference<T&> {
        typedef T type;
    };

    // Specialization for rvalue reference
    template<typename T>
    struct remove_reference<T&&> {
        typedef T type;
    };

} // namespace std

#endif // TYPE_TRAITS_HPP
