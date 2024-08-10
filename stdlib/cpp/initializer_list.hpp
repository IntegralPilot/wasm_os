#ifndef INITIALIZER_LIST_HPP
#define INITIALIZER_LIST_HPP

template <typename T>
class initializer_list {
public:
    using value_type = T;
    using reference = const T&;
    using const_reference = const T&;
    using size_type = size_t;
    using iterator = const T*;
    using const_iterator = const T*;

private:
    const T* data_;
    size_type size_;

public:
    // Default constructor (required for some compilers)
    initializer_list() : data_(nullptr), size_(0) {}

    // Constructor from array (used internally by the compiler)
    initializer_list(const T* data, size_type size) : data_(data), size_(size) {}

    // Member functions
    size_type size() const { return size_; }
    const_iterator begin() const { return data_; }
    const_iterator end() const { return data_ + size_; }
};

#endif // INITIALIZER_LIST_HPP