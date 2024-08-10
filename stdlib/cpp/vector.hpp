#ifndef VECTOR_HPP
#define VECTOR_HPP

#include <new.hpp>

#include "memory.hpp" // Assuming this provides malloc/free

namespace std {

template <typename T>
class vector {
public:
    using value_type = T;
    using size_type = size_t;

    using iterator = T*;
    using const_iterator = const T*;

private:
    T* data_;
    size_type size_;
    size_type capacity_;

    // Helper function for geometric growth
    void grow_capacity() {
        size_type new_capacity = capacity_ == 0 ? 1 : capacity_ * 2; 
        reserve(new_capacity);
    }

public:
    // Constructors
    vector() noexcept : data_(nullptr), size_(0), capacity_(0) {}

    explicit vector(size_type count, const T& value = T())
        : data_(allocate(count)), size_(count), capacity_(count)
    {
        for (size_type i = 0; i < size_; ++i) {
            data_[i] = value;
        }
    }

    // Copy Constructor
    vector(const vector& other) 
        : data_(allocate(other.size_)), size_(other.size_), capacity_(other.size_) 
    {
        for (size_type i = 0; i < size_; ++i) {
            data_[i] = other.data_[i]; 
        }
    }

    // Destructor
    ~vector() {
        deallocate(data_);
    }

    // Copy Assignment Operator
    vector& operator=(const vector& other) {
        if (this != &other) {
            T* new_data = allocate(other.size_);

            for (size_type i = 0; i < other.size_; ++i) {
                new_data[i] = other.data_[i];
            }

            deallocate(data_);
            data_ = new_data;
            size_ = other.size_;
            capacity_ = other.size_; 
        }
        return *this; 
    }

    // Element access
    T& operator[](size_type pos) { return data_[pos]; }
    const T& operator[](size_type pos) const { return data_[pos]; }

    T& at(size_type pos) {
        // You might want to handle out-of-range errors differently in an embedded system.
        if (pos >= size_) {
            for(;;); // Example: Halt the system
        }
        return data_[pos]; 
    }

    const T& at(size_type pos) const {
        if (pos >= size_) {
            for(;;); 
        }
        return data_[pos];
    }

    T& front() { return data_[0]; }
    const T& front() const { return data_[0]; }

    T& back() { return data_[size_ - 1]; }
    const T& back() const { return data_[size_ - 1]; } 

    // Iterators
    iterator begin() { return data_; }
    iterator end() { return data_ + size_; }

    const_iterator begin() const { return data_; }
    const_iterator end() const { return data_ + size_; }

    // Capacity
    size_type size() const noexcept { return size_; }
    bool empty() const noexcept { return size_ == 0; }
    size_type capacity() const noexcept { return capacity_; }

    void shrink_to_fit() {
        if (size_ < capacity_) {
            T* new_data = allocate(size_);

            if (!new_data) {
                for(;;); 
            }

            for (size_type i = 0; i < size_; ++i) {
                new_data[i] = std::move(data_[i]); 
            }

            deallocate(data_);
            data_ = new_data;
            capacity_ = size_;
        }
    }

    void reserve(size_type new_capacity) {
        if (new_capacity > capacity_) {
            T* new_data = allocate(new_capacity);

            if (!new_data) {
                for(;;); 
            }

            for (size_type i = 0; i < size_; ++i) {
                new_data[i] = std::move(data_[i]); 
            }

            deallocate(data_); 
            data_ = new_data;
            capacity_ = new_capacity;
        }
    }

    // resize
    void resize(size_type count) {
        if (count < size_) {
            for (size_type i = count; i < size_; ++i) {
                data_[i].~T(); // Call destructors of the elements
            }
            size_ = count;
        } else if (count > size_) {
            if (count > capacity_) {
                reserve(count);
            }
            for (size_type i = size_; i < count; ++i) {
                new (&data_[i]) T(); // Default construct in-place
            }
            size_ = count;
        }
    }

    // Modifiers
    void push_back(const T& value) { 
        if (size_ == capacity_) {
            grow_capacity();
        }
        data_[size_] = value;
        ++size_;
    }

    void pop_back() {
        if (size_ > 0) {
            --size_;
            data_[size_].~T(); // Call the destructor of the last element
        }
    }

    template <typename... Args>
    void emplace_back(Args&&... args) {
        if (size_ == capacity_) {
            grow_capacity();
        }
        new (&data_[size_]) T(std::forward<Args>(args)...); // Construct in-place
        ++size_;
    }

    void clear() noexcept {
        for (size_type i = 0; i < size_; ++i) {
            data_[i].~T(); // Call destructors of the elements
        }
        size_ = 0; 
    }

iterator insert(iterator pos, const T& value) {
    size_type index = pos - begin();
    if (size_ == capacity_) {
        grow_capacity();
        pos = begin() + index; // Recalculate position after capacity change
    }

    // Shift elements to the right to make space for new element
    for (size_type i = size_; i > index; --i) {
        data_[i] = std::move(data_[i - 1]);
    }

    data_[index] = value;
    ++size_;
    return begin() + index;
}


    iterator insert(iterator pos, iterator start, iterator end) {
        size_type count = end - start;
        size_type index = pos - begin();
        if (size_ + count > capacity_) {
            size_type new_capacity = size_ + count;
            T* new_data = allocate(new_capacity);

            if (!new_data) {
                for(;;); 
            }

            for (size_type i = 0; i < index; ++i) {
                new_data[i] = std::move(data_[i]);
            }

            for (size_type i = 0; i < count; ++i) {
                new (&new_data[index + i]) T(std::move(start[i]));
            }

            for (size_type i = index; i < size_; ++i) {
                new (&new_data[index + count + i]) T(std::move(data_[i]));
            }

            deallocate(data_);
            data_ = new_data;
            size_ += count;
            capacity_ = new_capacity;
        } else {
            for (size_type i = size_ + count - 1; i >= index + count; --i) {
                data_[i] = std::move(data_[i - count]);
            }

            for (size_type i = 0; i < count; ++i) {
                new (&data_[index + i]) T(std::move(start[i]));
            }

            size_ += count;
        }
        return begin() + index;
    }

    // range insert at pos 0
    void insert(iterator start, iterator end) {
        insert(0, start, end);
    }

    iterator erase(iterator pos) {
        if (pos != end()) {
            for (iterator it = pos; it != end() - 1; ++it) {
                *it = std::move(*(it + 1));
            }
            --size_;
            data_[size_].~T(); // Call the destructor of the last element
        }
        return pos; 
    }

    iterator erase(iterator first, iterator last) {
        size_type count = last - first;
        if (count > 0) {
            for (iterator it = first; it != end() - count; ++it) {
                *it = std::move(*(it + count));
            }
            for (iterator it = end() - count; it != end(); ++it) {
                it->~T(); // Call destructors of the elements
            }
            size_ -= count;
        }
        return first; 
    }

    // Other
    T* data() noexcept { return data_; }
    const T* data() const noexcept { return data_; }

private:
    static T* allocate(size_type n) {
        if (n > 0) {
            T* ptr = static_cast<T*>(::malloc(n * sizeof(T)));
            if (!ptr) {
                // Handle allocation failure 
                for(;;); 
            }
            return ptr;
        }
        return nullptr;
    }

    static void deallocate(T* ptr) noexcept {
        ::free(ptr);
    }
};

} // namespace std

#endif // VECTOR_HPP