#ifndef _TIME_H
#define _TIME_H

int timesinceboot();
int cputime();

typedef time_t long int;
typedef clock_t long int;

clock_t clock() {
    return cputime();
}

#endif

