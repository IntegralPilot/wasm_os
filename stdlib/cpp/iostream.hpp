// This function should be provided by the kernel
// We don't define it so that the clang makes the code dynamically link to it at runtime
extern "C" void putchar(int i);

namespace std {

class ostream {
public:
    ostream& operator<<(int value) {
        char buffer[12];
        itoa(value, buffer);
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

    void puts(const char* str) {
        while (*str) {
            putchar(*str);
            str++;
        }
    }
};

ostream cout;

char endl = '\n';
}