// C Program to print the fibonacci series using iteration
// (loops)
#include <stdio.h>
#include <stdlib.h>

// function to print fibonacci series
void printFib(int n)
{
    if (n <= 1) {
        printf("Invalid Number of terms\n");
        return;
    } else if (n == 0) {
        printf("No terms\n");
        return;
    }

    // when number of terms is greater than 0
    long long int prev1 = 1;
    long long int prev2 = 0;

    // for loop to print fibonacci series
    for (int i = 1; i <= n; i++) {
        if (i > 2) {
            long long int num = prev1 + prev2;
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

int main(int argc, char* argv[])
{
    // check if number of arguments is 2
    if (argc != 2) {
        printf("Usage: %s <number of terms>\n", argv[0]);
        return 1;
    }

    // convert string to integer
    int n = atoi(argv[1]);

    // print fibonacci series
    printFib(n);

    return 0;
}