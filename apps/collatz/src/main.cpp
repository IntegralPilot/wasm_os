#include <iostream.hpp>
#include <cstdlib.hpp>

#include "collatz.cpp"

int main(int argc, char *argv[]) {
  // usage: <binary> <number>
  // extract the number from args and run the Collatz Conjecture for X numbers

  if (argc != 2) {
    std::cout << "Usage: " << argv[0] << " <number>" << std::endl;
    return 1;
  }

  char *num = argv[1];

  int number = std::atoi(num);

  std::cout << "Running Collatz Conjecture for " << number << " numbers" << std::endl;

  std::cout << "1 is not a break!" << std::endl;

  int reps = number - 1;
  
  CollatzSolver thread1((int) 2, CollatzAlgorithm {
    true,
    true,
    true,
    true,
    true
  });
  for (int i = 0; i < reps; i++) {
    if (thread1.solve()) {
      std::cout << thread1.getCurrentNumber() << " is a break!" << std::endl;
      return 0;
    } else {
      std::cout << (thread1.getCurrentNumber() - 1) << " is not a break!" << std::endl;
    }
  }
}