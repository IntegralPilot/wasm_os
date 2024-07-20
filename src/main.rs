#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::vec;
use blog_os::serial_println;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use lazy_static::lazy_static;

entry_point!(kernel_main);

struct WasmOSLogListener;

lazy_static! {
    static ref WATCHING_CHANNELS: alloc::vec::Vec<&'static str> = vec![""];
}

impl breadcrumbs::LogListener for WasmOSLogListener {
    fn on_log(&mut self, log: breadcrumbs::Log) {
        #[cfg(not(debug_assertions))]
        if log.level.is_at_least(LogLevel::Warn) {
            serial_println!("{}", log);
        } else {
            log.remove();
        }
        #[cfg(debug_assertions)]
        if WATCHING_CHANNELS.contains(&log.channel.as_str()) {
            serial_println!("{}", log);
        } 
        #[cfg(debug_assertions)]
        if !log.level.is_at_least(breadcrumbs::LogLevel::Warn) {
            log.remove();
        }
    }
}
    


fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use blog_os::allocator;
    use blog_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    blog_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    loop {};
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use blog_os::println;

    println!("{}", info);
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
