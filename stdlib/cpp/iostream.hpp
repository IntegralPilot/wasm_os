#ifndef IOSTREAM_HPP
#define IOSTREAM_HPP

// These functions are implemented in the kernel.
// State the signatures but don't define them so that clang will make the .wasm file dynamically link to them at runtime.
extern "C" void putchar(int i);
extern "C" int getchar();

// serial versions of these functions
extern "C" void s_putchar(int i);

int getchar_safe() {
    int c = getchar();
    while (c == '\0') {
        c = getchar();
    }
    return c;
}

    void itoa(int num, char* str) {
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

    void llitoa(long long int num, char* str) {
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

namespace std {

class ostream {
public:
    ostream& operator<<(int value) {
        char buffer[12];
        itoa(value, buffer);
        puts(buffer);
        return *this;
    }

    ostream& operator<<(long long int value) {
        char buffer[21];
        llitoa(value, buffer);
        puts(buffer);
        return *this;
    }

    ostream& operator<<(const char* value) {
        puts(value);
        return *this;
    }

    ostream& operator<<(char value) {
        putchar(value);
        return *this;
    }

private:
    void puts(const char* str) {
        while (*str) {
            putchar(*str);
            str++;
        }
    }
};

ostream cout;

class s_ostream {
public:
    s_ostream& operator<<(int value) {
        char buffer[12];
        itoa(value, buffer);
        s_puts(buffer);
        return *this;
    }

    s_ostream& operator<<(const char* value) {
        s_puts(value);
        return *this;
    }

    s_ostream& operator<<(char value) {
        s_putchar(value);
        return *this;
    }
private:
    void s_puts(const char* str) {
        while (*str) {
            s_putchar(*str);
            str++;
        }
    }
};


s_ostream s_cout;

char endl = '\n';

class istream {
public:
    istream& operator>>(int& value) {
        value = 0;
        int sign = 1;
        char c = getchar_safe();
        while (c == ' ' || c == '\t' || c == '\n') {
            c = getchar_safe();
        }
        if (c == '-') {
            sign = -1;
            c = getchar_safe();
        }
        while (c >= '0' && c <= '9') {
            value = value * 10 + c - '0';
            c = getchar_safe();
        }
        value *= sign;
        return *this;
    }

    istream& operator>>(char& value) {
        char c = getchar_safe();
        value = c;
        return *this;
    }

    istream& operator>>(char* value) {
    char c = '\0';
    int i = 0; // Start at 0 instead of -1
    while (c != '\n') {
        c = getchar_safe();
        if (c == '\x08') {
            if (i > 0) {
                i--; // remove the character before the backspace character
            }
        } else {
            if (c == '\n') {
                break;
            }
            value[i++] = c; // Increment after assignment
        }
    }
    value[i] = '\0'; // Null-terminate the string
    return *this;
}
};

istream cin;

} // namespace std

#endif // IOSTREAM_HPP
