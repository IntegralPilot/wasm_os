#ifndef VECTOR_HPP
#define VECTOR_HPP

#include <new.hpp>
#include <utility.hpp>

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
            new (&data_[i]) T(value); // Use placement new for construction
        }
    }

    // Copy Constructor
    vector(const vector& other)
        : data_(allocate(other.size_)), size_(other.size_), capacity_(other.size_)
    {
        for (size_type i = 0; i < size_; ++i) {
            new (&data_[i]) T(other.data_[i]); // Use placement new for construction
        }
    }

    // Destructor
    ~vector() {
        clear(); // Destroy elements
        deallocate(data_); // Deallocate memory
    }

    // Copy Assignment Operator
    vector& operator=(const vector& other) {
        if (this != &other) {
            // A simple but not fully exception-safe implementation
            clear();
            deallocate(data_);

            data_ = allocate(other.capacity_);
            capacity_ = other.capacity_;
            size_ = other.size_;
            for (size_type i = 0; i < size_; ++i) {
                new (&data_[i]) T(other.data_[i]);
            }
        }
        return *this;
    }

    // Element access
    T& operator[](size_type pos) { return data_[pos]; }
    const T& operator[](size_type pos) const { return data_[pos]; }

    T& at(size_type pos) {
        if (pos >= size_) {
            for(;;);
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
                new (&new_data[i]) T(std::move(data_[i]));
                data_[i].~T();
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
                new (&new_data[i]) T(std::move(data_[i]));
                data_[i].~T();
            }

            deallocate(data_);
            data_ = new_data;
            capacity_ = new_capacity;
        }
    }

    void resize(size_type count) {
        if (count < size_) {
            for (size_type i = count; i < size_; ++i) {
                data_[i].~T();
            }
            size_ = count;
        } else if (count > size_) {
            if (count > capacity_) {
                reserve(count);
            }
            for (size_type i = size_; i < count; ++i) {
                new (&data_[i]) T();
            }
            size_ = count;
        }
    }

    // Modifiers
    void push_back(const T& value) {
        if (size_ == capacity_) {
            grow_capacity();
        }
        new (&data_[size_]) T(value);
        ++size_;
    }

    void pop_back() {
        if (size_ > 0) {
            --size_;
            data_[size_].~T();
        }
    }

    template <typename... Args>
    void emplace_back(Args&&... args) {
        if (size_ == capacity_) {
            grow_capacity();
        }
        new (&data_[size_]) T(std::forward<Args>(args)...);
        ++size_;
    }

    void clear() noexcept {
        for (size_type i = 0; i < size_; ++i) {
            data_[i].~T();
        }
        size_ = 0;
    }

    iterator insert(iterator pos, const T& value) {
        size_type index = pos - begin();
        if (size_ == capacity_) {
            grow_capacity();
            pos = begin() + index;
        }

        new (&data_[size_]) T();
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
                new (&new_data[i]) T(std::move(data_[i]));
            }
            for (size_type i = 0; i < count; ++i) {
                new (&new_data[index + i]) T(*(start + i));
            }
            for (size_type i = index; i < size_; ++i) {
                new (&new_data[index + count + i - index]) T(std::move(data_[i]));
            }

            clear();
            deallocate(data_);
            data_ = new_data;
            size_ = index + count + (size_ - index);
            capacity_ = new_capacity;
        } else {
            for (size_type i = size_ + count - 1; i >= index + count; --i) {
                data_[i] = std::move(data_[i - count]);
            }
            for (size_type i = 0; i < count; ++i) {
                new (&data_[index + i]) T(*(start + i));
            }
            size_ += count;
        }
        return begin() + index;
    }

    // range insert at pos 0
    void insert(iterator start, iterator end) {
        // FIX: Pass the actual beginning iterator, not a null pointer.
        insert(this->begin(), start, end);
    }

    iterator erase(iterator pos) {
        if (pos != end()) {
            for (iterator it = pos; it != end() - 1; ++it) {
                *it = std::move(*(it + 1));
            }
            --size_;
            data_[size_].~T();
        }
        return pos;
    }

    iterator erase(iterator first, iterator last) {
        size_type count = last - first;
        if (count > 0) {
            iterator write_pos = first;
            iterator read_pos = last;
            while (read_pos != end()) {
                *write_pos++ = std::move(*read_pos++);
            }
            for (iterator it = write_pos; it != end(); ++it) {
                it->~T();
            }
            size_ -= count;
        }
        return first;
    }

    T* data() noexcept { return data_; }
    const T* data() const noexcept { return data_; }

private:
    static T* allocate(size_type n) {
        if (n > 0) {
            void* raw_ptr = ::malloc(n * sizeof(T));
            if (!raw_ptr) {
                for(;;);
            }
            return static_cast<T*>(raw_ptr);
        }
        return nullptr;
    }

    static void deallocate(T* ptr) noexcept {
        ::free(ptr);
    }
};

} // namespace std

#endif // VECTOR_HPP