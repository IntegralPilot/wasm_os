#ifndef FUNCTIONAL_HPP
#define FUNCTIONAL_HPP

namespace std {

// std::less
template <typename T = void>
struct less {
    bool operator()(const T& lhs, const T& rhs) const {
        return lhs < rhs;
    }
};

// Specialization for void (to allow std::less<> with no arguments)
template <>
struct less<void> {
    template <typename T1, typename T2>
    bool operator()(const T1& lhs, const T2& rhs) const {
        return lhs < rhs;
    }
};

}  // namespace std

#endif // FUNCTIONAL_HPP