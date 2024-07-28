#include <iostream.hpp>
#include <cstring.hpp>
#include <cstdlib.hpp>

extern "C" void _start() {
    // implement a simple cli
    std::cout << "Welcome to Hydro CLI" << std::endl;
    std::cout << "Type 'help' to see built-in commands" << std::endl;

    while (true) {
        std::cout << "> ";
        char command[100];
        std::cin >> command;

        if (strcmp(command, "exit") == 0) {
            break;
        } else if (strcmp(command, "help") == 0) {
            std::cout << "Built-in commands:" << std::endl;
            std::cout << "help - show available commands" << std::endl;
            std::cout << "exit - exit the CLI" << std::endl;
        } else {
            int returncode = system(command);
            if (returncode == -1) {
                std::cout << "Command not found" << std::endl;
            } else if (returncode == -2) {
                std::cout << "System error running command." << std::endl;
            } else if (returncode != 0) {
                std::cout << "Command returned with error code " << returncode << std::endl;
            }
        }
    }
}