#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::ToString;
use alloc::sync::Arc;
use alloc::{format, vec};
use bootloader::{BootInfo, entry_point};
use breadcrumbs::{LogLevel, log};
use lazy_static::lazy_static;
use spin::Mutex;
use wasm_os::apps::register_app;
use wasm_os::apps::run_app;
use wasm_os::serial_println;

#[cfg(not(debug_assertions))]
use breadcrumbs::LogLevel;

entry_point!(kernel_main);

struct WasmOSLogListener;

lazy_static! {
    static ref WATCHING_CHANNELS: alloc::vec::Vec<&'static str> = vec![];
    static ref CURRENT_BYTE_IN_STDIN: Arc<Mutex<char>> = Arc::new(Mutex::new('\0'));
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
        if WATCHING_CHANNELS.contains(&log.channel.as_str())
            || log.level.is_at_least(breadcrumbs::LogLevel::Warn)
        {
            serial_println!("{}", log);
        } else {
            log.remove();
        }
    }
}

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Trace
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                log::Level::Error => log!(LogLevel::Error, "naive-log", record.args()),
                log::Level::Warn => log!(LogLevel::Warn, "naive-log", record.args()),
                log::Level::Info => log!(LogLevel::Info, "naive-log", record.args()),
                log::Level::Debug => log!(LogLevel::Info, "naive-log", record.args()),
                log::Level::Trace => log!(LogLevel::Verbose, "naive-log", record.args()),
            }
        }
    }

    fn flush(&self) {
        // re-init breadcrumbs to flush logs
        breadcrumbs::init!(WasmOSLogListener);
    }
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use wasm_os::allocator;
    use wasm_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    wasm_os::init();

    // we can't use breadcrumbs here yet because it requires the heap to be initialized
    // mimic logging for early bootup messages untill we can use breadcrumbs
    serial_println!("[bootup/Info] Kernel started");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    serial_println!("[bootup/Info] Heap initialized");

    breadcrumbs::init!(WasmOSLogListener);
    match log::set_logger(&SimpleLogger) {
        Ok(_) => log::set_max_level(log::LevelFilter::Trace),
        Err(e) => log!(
            LogLevel::Error,
            "naive-log",
            format!("Error setting logger: {:?}", e)
        ),
    }

    #[cfg(debug_assertions)]
    let channels_being_wachted = WATCHING_CHANNELS.len();
    if channels_being_wachted == 0 {
        serial_println!(
            "*** The logging system has now been initialised. No channels are being watched. To see logs below Warn level, add the desired channels to WATCHING_CHANNELS in main.rs. ***"
        );
    } else if channels_being_wachted == 1 {
        serial_println!(
            "*** The logging system has now been initialised. You are watching 1 channel. ***"
        );
    } else {
        serial_println!(
            "*** Breadcrumbs Logging has now been initialized. You are watching {} channels ***",
            channels_being_wachted
        );
    }
    #[cfg(not(debug_assertions))]
    serial_println!(
        "### Breadcrumbs Logging has now been initialized. Release mode is enabled, no logs below Warn will be shown. ###"
    );

    wasm_os::inode::init_dev();

    log!(LogLevel::Info, "bootup", "Inode system initialized");

    // Include the generated file
    include!(concat!(env!("OUT_DIR"), "/generated_apps.rs"));

    match run_app("cli-cpp") {
        Ok(_) => {
            panic!("Default app died.");
            // will implement shutdown later
        }
        Err(e) => {
            panic!("Error starting default app: {:?}", e);
        }
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use alloc::format;
    use wasm_os::{
        println,
        vga_buffer::{_println_with_color, Color},
    };

    let panic_info = format!("{}", info);
    serial_println!("Kernel Panic: {}", panic_info);
    //_clear_screen();
    println!();
    _println_with_color("Kernel Panic!", Color::LightRed);
    _println_with_color(panic_info.as_str(), Color::White);

    loop {}
}
