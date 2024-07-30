#include <stdlib.h>
#include <stdio.h>

void _start() {
    printf("Hello, World from Allocations in C on Hydro!\n");

    // demo malloc, free and realloc
    int* arr = (int*)malloc(5 * sizeof(int));
    for (int i = 0; i < 5; i++) {
        arr[i] = i;
    }

    // print the address of the memory we got
    printf("Address of the memory we got: %p\n", arr);

    for (int i = 0; i < 5; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");

    arr = (int*)realloc(arr, 10 * sizeof(int));
    for (int i = 5; i < 10; i++) {
        arr[i] = i;
    }

    for (int i = 0; i < 10; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");

    // print the address of the memory we got
    printf("Address of the memory we got: %p\n", arr);

    free(arr);

    printf("Allocations demo done!\n");
}