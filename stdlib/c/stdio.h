#ifndef _STDIO_H
#define _STDIO_H

#include <stdio.c>

int printf(const char *format, ...);
int vprintf(const char *format, va_list args);

#endif // STDIO_H