// C Program to print the fibonacci series using iteration
// (loops)
#include <stdio.h>

// function to print fibonacci series
void printFib(int n)
{
    if (n < 1) {
        printf("Invalid Number of terms\n");
        return;
    }

    // when number of terms is greater than 0
    int prev1 = 1;
    int prev2 = 0;

    // for loop to print fibonacci series
    for (int i = 1; i <= n; i++) {
        if (i > 2) {
            int num = prev1 + prev2;
            prev2 = prev1;
            prev1 = num;
            printf("%d ", num);
        }

        // for first two terms
        if (i == 1) {
            printf("%d ", prev2);
        }
        if (i == 2) {
            printf("%d ", prev1);
        }
        printf("\n");
    }
}

// driver code
int _start()
{

    int n = 10;
    printFib(n);
    return 0;
}
