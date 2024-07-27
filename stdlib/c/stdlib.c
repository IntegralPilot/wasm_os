int atoi(const char* str)
{
    int res = 0;
    int sign = 1;
    int i = 0;
    if (str[0] == '-') {
        sign = -1;
        i++;
    }
    for (; str[i] != '\0'; ++i) {
        res = res * 10 + str[i] - '0';
    }
    return sign * res;
}