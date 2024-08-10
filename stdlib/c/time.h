#ifndef _TIME_H
#define _TIME_H

typedef long int time_t;
typedef long int clock_t;

// Function declarations for external functions
int timesinceboot();  // Returns time since boot in microseconds
int cputime();        // Returns CPU time used in microseconds

// Returns the processor time consumed by the program
clock_t clock() {
    return cputime();
}

// Returns the current calendar time
// Assume that the machine booted at the Unix epoch
time_t time(time_t* t) {
    time_t current_time = (time_t)timesinceboot();
    if (t != NULL) {
        *t = current_time;
    }
    return current_time;
}

#endif // _TIME_H
