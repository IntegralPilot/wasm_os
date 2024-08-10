#ifndef _STRING_C
#define _STRING_C

int strcmp(const char* s1, const char* s2) {
    while (*s1 && *s2 && *s1 == *s2) {
        s1++;
        s2++;
    }
    // Return the difference between the current characters pointed to by s1 and s2
    // This correctly handles the case where one string might be shorter than the other
    return (unsigned char)*s1 - (unsigned char)*s2;
}

unsigned long strlen(const char* str) {
    unsigned long len = 0;
    while (*str) {
        len++;
        str++;
    }
    return len;
}

#endif // STRING_C