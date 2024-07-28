// C Program to print the fibonacci series using iteration
// (loops)
#include <stdio.h>
#include <stdlib.h>
#include <args.h>
#include <stddef.h>

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

void _start()
{

    // Retrieve command line arguments
    char* args = getargs(); // Assuming getargs() provides the command line as an array of strings

    // args will be like this
    // n a m e \0 a r g 1 \0 a r g 2 \0 a r g 3 \0 \0
    // ^
    // argv pointer

    // so calculate the number of args
    // once we have two \0 in a row, we know that we have reached the end of the args
    // also make a char* [] with the pointers to the start of each arg
    int argc = 0;
    int wasNull = 1;
    char* argv[100];
    argv[argc] = args;
    for (int i = 0; i < 1000; i++) {
        if (args[i] == '\0') {
            if (wasNull) {
                break;
            } else {
                wasNull = 1;
            }
            argc++;
            argv[argc] = &args[i + 1];
        } else if (wasNull) {
            wasNull = 0;
        }
    }

    // Call main function
    main(argc, argv);
}