#include <iostream.hpp>
#include <time.hpp>

extern "C" void _start() {
    std::cout << "The time since boot is: " << timesinceboot() << " microseconds." << std::endl;
    std::cout << "The time this app has been running is: " << cputime() << " microseconds." << std::endl;
}