#include <stdarg.h>
#include <string.h>
#include <stdint.h> // Include for uintptr_t

// This function is implemented in the kernel
// We don't define it so that the clang makes the code dynamically link to it at runtime
void putchar(int i);

// Use macros for digit conversion to avoid magic numbers
#define TO_DIGIT(n) ((n) + '0')

void itoa(int num, char *str) {
    int i = 0;
    int isNegative = 0;

    if (num == 0) {
        str[i++] = '0';
        str[i] = '\0';
        return;
    }

    if (num < 0) {
        isNegative = 1;
        num = -num;
    }

    while (num != 0) {
        int rem = num % 10;
        str[i++] = TO_DIGIT(rem); // Use macro
        num = num / 10;
    }

    if (isNegative) {
        str[i++] = '-';
    }

    // Reverse the string (could be optimized)
    for (int start = 0, end = i - 1; start < end; start++, end--) {
        char temp = str[start];
        str[start] = str[end];
        str[end] = temp;
    }

    str[i] = '\0';
}

// This function is almost identical to itoa. 
// Consider refactoring to avoid code duplication
void llitoa(long long int num, char *str) {
    int i = 0;
    int isNegative = 0;

    if (num == 0) {
        str[i++] = '0';
        str[i] = '\0';
        return;
    }

    if (num < 0) {
        isNegative = 1;
        num = -num;
    }

    while (num != 0) {
        int rem = num % 10;
        str[i++] = TO_DIGIT(rem); // Use macro
        num = num / 10;
    }

    if (isNegative) {
        str[i++] = '-';
    }

    // Reverse the string (could be optimized)
    for (int start = 0, end = i - 1; start < end; start++, end--) {
        char temp = str[start];
        str[start] = str[end];
        str[end] = temp;
    }

    str[i] = '\0';
}

void puts(const char *str) {
    while (*str) {
        putchar(*str);
        str++;
    }
}

int vprintf(const char *format, va_list args) {
    int count = 0;

    while (*format) {
        if (*format == '%') {
            format++;
            switch (*format) {
                case 'd': {
                    int i = va_arg(args, int);
                    char s[12]; 
                    itoa(i, s);
                    puts(s);
                    count += strlen(s);
                    break;
                }
                case 's': {
                    char *s = va_arg(args, char *);
                    puts(s);
                    count += strlen(s);
                    break;
                }
                case 'c': {
                    char c = (char)va_arg(args, int); 
                    putchar(c);
                    count++;
                    break;
                }
                case 'p': {
                    // Use uintptr_t for pointer conversion
                    uintptr_t p = va_arg(args, uintptr_t);
                    char s[21];
                    s[0] = '0';
                    s[1] = 'x';
                    // Use a loop and bit shifting for hex conversion
                    for (int i = sizeof(p) * 2 - 1; i >= 0; --i) {
                        int digit = (p >> (i * 4)) & 0xF;
                        s[2 + sizeof(p) * 2 - 1 - i] = (digit < 10) ? TO_DIGIT(digit) : (digit - 10 + 'a');
                    }
                    s[sizeof(s) - 1] = '\0';
                    puts(s);
                    count += strlen(s);
                    break;
                }
case 'l': {
    format++;
    if (*format == 'l' && *(format + 1) == 'd') { 
        // Only increment format once here
        format++; 
        long long int i = va_arg(args, long long int);
        char s[21];
        llitoa(i, s);
        puts(s);
        count += strlen(s);
    } else {
        // Handle invalid specifier
        putchar('%');
        putchar('l');
        putchar(*format);
        count += 3;
    }
    break;
}
                default: 
                    // Handle other invalid specifiers
                    putchar('%');
                    putchar(*format);
                    count += 2;
            }
        } else {
            putchar(*format);
            count++;
        }
        format++;
    }

    return count;
}

int printf(const char *format, ...) {
    va_list args;
    va_start(args, format);
    int count = vprintf(format, args);
    va_end(args);
    return count;
}