#![no_std]

use hydro_std::println;

#[no_mangle]
pub extern "C" fn _start() {
    println("Hello, World from Rust on Hydro!");
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}