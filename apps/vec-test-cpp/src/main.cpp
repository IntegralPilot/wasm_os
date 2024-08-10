#include <vector.hpp>
#include <iostream.hpp>
#include <cstdlib.hpp>  // for rand and srand
#include <ctime.hpp>    // for time
#include <new.hpp>  // for std::badalloc

int main() {
    std::srand(std::time(nullptr));

    // generate 10 random numbers to use in our testing
    std::vector<int> original_numbers;
    for (int i = 0; i < 10; i++) {
        original_numbers.push_back(std::rand() % 100);
    }

    std::vector<int> numbers;

    // test push_back
    for (int i = 0; i < 10; i++) {
        numbers.push_back(original_numbers[i]);
    }

    // check if the numbers are the same
    for (int i = 0; i < 10; i++) {
        if (numbers[i] != original_numbers[i]) {
            std::cout << "Error: numbers[" << i << "] = " << numbers[i] << " != " << original_numbers[i] << std::endl;
            return 1;
        }
    }

    std::cout << "Push back test passed" << std::endl;

    // test pop_back
    for (int i = 0; i < 10; i++) {
        numbers.pop_back();
    }

    // check if the vector is empty
    if (!numbers.empty()) {
        std::cout << "Error: vector is not empty after calling pop_back 10 times" << std::endl;
        return 1;
    }

    std::cout << "Pop back test passed" << std::endl;

    // test reserve
    numbers.reserve(10);
    if (numbers.capacity() < 10) {
        std::cout << "Error: capacity is less than 10 after calling reserve(10)" << std::endl;
        return 1;
    }

    std::cout << "Reserve test passed" << std::endl;

    // test resize
    numbers.resize(10);
    if (numbers.size() != 10) {
        std::cout << "Error: size is not 10 after calling resize(10)" << std::endl;
        return 1;
    }

    std::cout << "Resize test passed" << std::endl;

    // test clear
    numbers.clear();
    if (!numbers.empty()) {
        std::cout << "Error: vector is not empty after calling clear" << std::endl;
        return 1;
    }

    std::cout << "Clear test passed" << std::endl;

    // test insert
    numbers.insert(original_numbers.begin(), original_numbers.end());

    // check if the numbers are the same
    for (int i = 0; i < 10; i++) {
        if (numbers[i] != original_numbers[i]) {
            std::cout << "Error: numbers[" << i << "] = " << numbers[i] << " != " << original_numbers[i] << std::endl;
            return 1;
        }
    }

    std::cout << "Insert test passed" << std::endl;

    // test erase
    numbers.erase(numbers.begin(), numbers.end());

    // check if the vector is empty
    if (!numbers.empty()) {
        std::cout << "Error: vector is not empty after calling erase" << std::endl;
        return 1;
    }

    std::cout << "Erase test passed" << std::endl;

    return 0;
}

extern "C" void _start() {
    main();
}
