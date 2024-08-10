#ifndef _STDLIB_H
#define _STDLIB_H

#include "stdlib.c"

int atoi(const char* str);
void* realloc(void* ptr, size_t size);

// rng
int rand();
void srand(unsigned int seed);

#endif // STDLIB_H