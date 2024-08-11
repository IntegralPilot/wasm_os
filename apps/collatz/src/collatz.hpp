// Part of a Library by Reeves, Verheijden and Lamb for a Collatz Conjecture Solver

#include <vector.hpp>

struct CollatzAlgorithm {
  bool useModulo;
  bool useDivideTwoCheck;
  bool useGlobalCache;
  bool useCacheTrim;
  bool useOnlyOddNumbers;
};


class CollatzSolver {
  void step();
  int stepNumber;
  int currentNumber;
  CollatzAlgorithm algo;
  std::vector<int> not_break_cache = {};
  std::vector<int> been_to_cache = {};
 public:
  CollatzSolver(int initialNumber, CollatzAlgorithm newAlgo): currentNumber(initialNumber), stepNumber(initialNumber), algo(newAlgo) {};
  bool solve();
  int getCurrentNumber() {
    return currentNumber;
  }
  int hops;
};
