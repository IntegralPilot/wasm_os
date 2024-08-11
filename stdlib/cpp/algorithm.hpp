#ifndef _ALGORITHM_HPP
#define _ALGORITHM_HPP

#include <utility.hpp>

namespace std {
    // The find algorithm
    template <class InputIterator, class T>
    InputIterator find(InputIterator first, InputIterator last, const T& value) {
        while (first != last) {
            if (*first == value) {
                return first;
            }
            ++first;
        }
        return last;
    }
    
    // The remove algorithm
    template <class ForwardIterator, class T>
    ForwardIterator remove(ForwardIterator first, ForwardIterator last, const T& value) {
        first = std::find(first, last, value);
        if (first == last) return first;
        ForwardIterator i = first;
        ++i;
        while (i != last) {
            if (*i != value) {
                *first++ = std::move(*i);
            }
            ++i;
        }
        return first;
    }
}

#endif
