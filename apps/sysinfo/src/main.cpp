#include <iostream.hpp>
#include <ctime.hpp>

extern "C" void _start() {
    std::cout << "The time since boot is: " << std::time(nullptr) << " microseconds." << std::endl;
    std::cout << "The time this app has been running is: " << std::clock() << " microseconds." << std::endl;
}