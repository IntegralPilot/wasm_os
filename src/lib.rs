#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use spin::Mutex;
use vga_buffer::_backspace;

static CURRENT_BYTE_IN_STDIN: Mutex<char> = Mutex::new('\0');
static ALLOWED_BACKSPACES: Mutex<u8> = Mutex::new(0);

pub fn reset_allowed_backspaces() {
    *ALLOWED_BACKSPACES.lock() = 0;
}

pub fn set_current_byte_in_stdin(byte: char) {
    {
        let mut allowed_backspaces = ALLOWED_BACKSPACES.lock();

        if byte != '\x08' {
            print!("{}", byte);
            *allowed_backspaces += 1;
        } else if *allowed_backspaces > 0 {
            _backspace();
            *allowed_backspaces -= 1;
        }
    }
    {
        *CURRENT_BYTE_IN_STDIN.lock() = byte;
    }
}

pub fn get_current_byte_in_stdin() -> char {
    let mut stdin = CURRENT_BYTE_IN_STDIN.lock();
    let current_byte = *stdin;
    // set it to \0 so we know it's been read
    *stdin = '\0';
    current_byte
}

extern crate alloc;
use core::panic::PanicInfo;

pub mod allocator;
pub mod apps;
pub mod gdt;
pub mod inode;
pub mod interrupts;
pub mod keyboard;
pub mod memory;
pub mod serial;
pub mod vga_buffer;
pub mod wasm;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
use bootloader::{BootInfo, entry_point};

#[cfg(test)]
entry_point!(test_kernel_main);

/// Entry point for `cargo xtest`
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
