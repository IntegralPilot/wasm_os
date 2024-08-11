// Part of a Library by Reeves, Verheijden and Lamb for a Collatz Conjecture Solver

#include <iostream.hpp>
#include <vector.hpp>
#include <algorithm.hpp>
#include "collatz.hpp"

void CollatzSolver::step() {
    hops++;
    if (algo.useOnlyOddNumbers) {
        hops++;
    }
    if (stepNumber % 2 == 0) {
        // it is even
        stepNumber /= 2;
    } else {
        // it is odd
        stepNumber *= 3;
        stepNumber++;
    }
}

bool CollatzSolver::solve() {
    if (algo.useOnlyOddNumbers && currentNumber % 2 == 0) {
        // it's even, so it's not a break
        currentNumber++;
        return false;
    }
    stepNumber = currentNumber;
    while (stepNumber != 1) {
        step();
        // Use std::find instead of custom contains
        if (std::find(been_to_cache.begin(), been_to_cache.end(), stepNumber) != been_to_cache.end()) {
            // it's a break!
            std::cout << "The number that breaks the Collatz Conjecture is " << currentNumber << std::endl;
            return true;
        } else if (algo.useDivideTwoCheck && stepNumber < currentNumber) {
            // it's *not* a break, all lower numbers definitely reach one!
            not_break_cache.insert(not_break_cache.end(), been_to_cache.begin(), been_to_cache.end());
            been_to_cache.clear();
            currentNumber++;
            return false;
        } else if (algo.useGlobalCache && std::find(not_break_cache.begin(), not_break_cache.end(), stepNumber) != not_break_cache.end()) {
            // it's *not* a break, we've checked and this reaches 1!
            not_break_cache.insert(not_break_cache.end(), been_to_cache.begin(), been_to_cache.end());
            been_to_cache.clear();
            currentNumber++;
            return false;
        }
        been_to_cache.push_back(stepNumber);
    }
    // it's *not* a break
    if (algo.useGlobalCache) {
        not_break_cache.insert(not_break_cache.end(), been_to_cache.begin(), been_to_cache.end());
    }
    been_to_cache.clear();
    currentNumber++;
    if (algo.useCacheTrim) {
        for (auto it = not_break_cache.begin(); it != not_break_cache.end(); ) {
            if (*it >= currentNumber) {
                // Use std::remove and vector::erase instead of custom remove
                it = not_break_cache.erase(it);
            } else {
                ++it;
            }
        }
    }
    return false;
}
