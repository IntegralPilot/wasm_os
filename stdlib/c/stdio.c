#include <stdarg.h>
#include <string.h>

// This function is implemented in the kernel
// We don't define it so that the clang makes the code dynamically link to it at runtime
void putchar(int i);

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
        str[i++] = rem + '0';
        num = num / 10;
    }

    if (isNegative)
        str[i++] = '-';

    str[i] = '\0';

    // Reverse the string
    int start = 0;
    int end = i - 1;
    while (start < end) {
        char temp = str[start];
        str[start] = str[end];
        str[end] = temp;
        start++;
        end--;
    }
}

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
        str[i++] = rem + '0';
        num = num / 10;
    }

    if (isNegative)
        str[i++] = '-';

    str[i] = '\0';

    // Reverse the string
    int start = 0;
    int end = i - 1;
    while (start < end) {
        char temp = str[start];
        str[start] = str[end];
        str[end] = temp;
        start++;
        end--;
    }
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
                    char c = (char)va_arg(args, int); // promote char to int for va_arg
                    putchar(c);
                    count++;
                    break;
                }
                case 'p': {
                    // Print the address in hexadecimal
                    unsigned long p = va_arg(args, unsigned long);
                    char s[21];
                    s[0] = '0';
                    s[1] = 'x';
                    llitoa(p, s + 2);
                    puts(s);
                    count += strlen(s);
                    break;
                }
                case 'l': {
                    format++;
                    if (*format == 'l' && *(format + 1) == 'd') { // "lld"
                        format += 2;
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
