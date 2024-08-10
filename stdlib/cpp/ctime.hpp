#ifndef _CTIME_HPP
#define _CTIME_HPP


extern "C" {
    int timesinceboot();  // Returns time since boot in microseconds
    int cputime();        // Returns CPU time used in microseconds
}

namespace std {

    
typedef long int time_t;
typedef long int clock_t;

// Returns the processor time consumed by the program
clock_t clock() {
    return cputime();
}

// Returns the current calendar time
// Assume that the machine booted at the Unix epoch
time_t time(time_t* t) {
    time_t current_time = static_cast<time_t>(timesinceboot());
    if (t != nullptr) {
        *t = current_time;
    }
    return current_time;
}

} // namespace std

#endif // _CTIME_HPP
