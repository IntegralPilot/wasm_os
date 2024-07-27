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

        // turn each char in command to an int, and s_cout them like this: [int, int, int, int, int, ...]
        std::s_cout << "[";
        for (int i = 0; i < strlen(command); i++) {
            std::s_cout << (int)command[i] << ", ";
        }
        std::s_cout << "]" << std::endl;

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