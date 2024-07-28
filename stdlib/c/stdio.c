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

int printf(const char *format, ...) {
    va_list args;
    va_start(args, format);

    int count = 0;

    while (*format) {
        if (*format == '%') {
            format++;
            if (strcmp(format, "lld") == 0) {
                long long int i = va_arg(args, long long int);
                char s[21];
                llitoa(i, s);
                puts(s);
                while (s[count] != '\0') {
                    count++;
                }
                format += 2; // Skip "lld"
            } else {
                switch (*format) {
                    case 'd': {
                        int i = va_arg(args, int);
                        char s[12];
                        itoa(i, s);
                        puts(s);
                        while (s[count] != '\0') {
                            count++;
                        }
                        break;
                    }
                    case 's': {
                        char *s = va_arg(args, char *);
                        puts(s);
                        while (*s) {
                            count++;
                            s++;
                        }
                        break;
                    }
                    case 'c': {
                        char c = va_arg(args, int);
                        putchar(c);
                        count++;
                        break;
                    }
                    default:
                        putchar('%');
                        putchar(*format);
                        count += 2;
                }
            }
        } else {
            putchar(*format);
            count++;
        }
        format++;
    }

    va_end(args);
    return count;
}