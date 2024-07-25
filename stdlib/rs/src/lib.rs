#![no_std]

extern "C" {
    fn putchar(c: i32);
}

pub fn print(s: &str) {
    for c in s.chars() {
        unsafe {
            putchar(c as i32);
        }
    }
}

pub fn println(s: &str) {
    print(s);
    print("\n");
}
