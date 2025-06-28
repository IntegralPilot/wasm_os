use core::convert::TryInto;

use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use breadcrumbs::log;
use lazy_static::lazy_static;
use rand::{RngCore, SeedableRng};
use spin::Mutex;
use tinywasm::{Extern, FuncContext, MemoryStringExt};

use crate::{
    inode::get_inode,
    interrupts::{NUMBER_OF_TIMER_INTERRUPTS, NUMBER_OF_TIMER_INTERRUPTS_SINCE_RESET},
    println,
    vga_buffer::_clear_screen,
};

#[derive(Clone, Copy)]
struct Allocation {
    ptr: usize,
    size: usize,
}

#[derive(Clone)]
struct AllocCommandReturn(Vec<Allocation>, usize);

fn malloc(allocations: Vec<Allocation>, size: usize) -> Result<AllocCommandReturn, String> {
    // A simple but robust bump allocator.
    // It finds the highest allocated address and allocates memory immediately after it.
    let ptr = if let Some(last_alloc) = allocations.iter().max_by_key(|a| a.ptr + a.size) {
        // Find the end of the highest allocation.
        last_alloc.ptr + last_alloc.size
    } else {
        // This is the first allocation. Start at address 1 to avoid returning 0 (which is nullptr in C/C++).
        1
    };

    let mut new_allocations = allocations.clone();
    new_allocations.push(Allocation { ptr, size });
    Ok(AllocCommandReturn(new_allocations, ptr))
}

fn free(allocations: Vec<Allocation>, ptr: usize) -> Result<Vec<Allocation>, String> {
    let mut new_allocations = Vec::new();
    for allocation in allocations.iter() {
        if allocation.ptr == ptr {
            continue;
        }
        new_allocations.push(*allocation);
    }
    Ok(new_allocations)
}

pub fn run_from_bytes(args: String, bytes: &[u8]) -> Result<(), String> {
    log!(
        breadcrumbs::LogLevel::Info,
        "wasm",
        format!("Running wasm module with args: {}", args)
    );
    static MEMORY_ALLOCATIONS: Mutex<Vec<Allocation>> = Mutex::new(Vec::new());
    log!(
        breadcrumbs::LogLevel::Verbose,
        "wasm-init",
        "Defined memory allocations"
    );
    lazy_static! {
        static ref RNG: Mutex<rand::rngs::SmallRng> =
            Mutex::new(rand::rngs::SmallRng::seed_from_u64(1u64));
    }
    log!(breadcrumbs::LogLevel::Verbose, "wasm-init", "Setup RNG");
    let module = match tinywasm::Module::parse_bytes(bytes) {
        Ok(module) => module,
        Err(err) => return Err(err.to_string()),
    };
    log!(
        breadcrumbs::LogLevel::Verbose,
        "wasm-init",
        "Parsed wasm module"
    );
    let mut store = tinywasm::Store::default();
    let mut imports = tinywasm::Imports::new();

    match imports.define(
        "env",
        "putchar",
        Extern::typed_func(|_: FuncContext<'_>, v: i32| {
            log!(
                breadcrumbs::LogLevel::Verbose,
                "guest-abi-calls",
                format!("putchar called with value: {}", v)
            );
            let node = get_inode("/dev/stdout");
            if let Some(inode) = node {
                inode.write_inputreciever(vec![v as u8]);
            }
            Ok(())
        }),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    match imports.define(
        "env",
        "s_putchar",
        Extern::typed_func(|_: FuncContext<'_>, v: i32| {
            log!(
                breadcrumbs::LogLevel::Verbose,
                "guest-abi-calls",
                format!("s_putchar called with value: {}", v)
            );
            let node = get_inode("/dev/serial0");
            if let Some(inode) = node {
                inode.write_inputreciever(vec![v as u8]);
            }
            Ok(())
        }),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    match imports.define(
        "env",
        "getchar",
        Extern::typed_func(|_: FuncContext<'_>, _: ()| {
            // we notably DON'T log getchar in guest-abi-calls as it is called very frequently
            let node = get_inode("/dev/stdin");
            if let Some(inode) = node {
                let data = inode.read_outputter();
                if let Some(data) = data {
                    return Ok(data[0] as i32);
                }
            }
            Ok(-1)
        }),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    match imports.define(
        "env",
        "abort",
        Extern::typed_func(|_: FuncContext<'_>, _: (i32, i32, i32, i32)| {
            println!("Progam aborted.");
            Ok(())
        }),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    match imports.define(
        "env",
        "runapp",
        Extern::typed_func(|mut context: FuncContext<'_>, pointer: i32| {
            log!(
                breadcrumbs::LogLevel::Verbose,
                "guest-abi-calls",
                format!("runapp called with pointer: {}", pointer)
            );
            let name = context.exported_memory("memory");
            match name {
                Ok(name) => {
                    let name = name.load_cstring_until_nul(pointer as usize, 100);
                    match name {
                        Ok(name) => match crate::apps::run_app(name.to_str().unwrap()) {
                            Ok(_) => Ok(0),
                            Err(i) => Ok(i),
                        },
                        Err(_) => Ok(-1),
                    }
                }
                Err(_) => Ok(-2),
            }
        }),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    match imports.define(
        "env",
        "getargs",
        Extern::typed_func(move |mut context: FuncContext<'_>, _: ()| {
            log!(
                breadcrumbs::LogLevel::Verbose,
                "guest-abi-calls",
                "getargs called"
            );
            let mut memory = match context.exported_memory_mut("memory") {
                Ok(memory) => memory,
                Err(_) => return Ok(-1),
            };
            let args_clone = args.clone();
            let args = args_clone.split(' ').collect::<Vec<&str>>().join("\0");
            let args = [args.as_bytes(), &[0u8], &[0u8]].concat();
            // turn the Vec<u8> into a &[u8]
            let args = args.as_slice();
            let args_len = args.len();
            let mut memory_allocations = MEMORY_ALLOCATIONS.lock();
            let offset = match malloc((*memory_allocations.clone()).to_vec(), args_len) {
                Ok(offset) => {
                    *memory_allocations = offset.0;
                    offset.1
                }
                Err(_) => return Ok(-1),
            };
            match memory.store(offset, args_len, args) {
                Ok(_) => return Ok(offset as i32),
                Err(_) => return Ok(-1),
            }
        }),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    match imports.define(
        "env",
        "timesinceboot",
        Extern::typed_func(|_: FuncContext<'_>, _: ()| {
            log!(
                breadcrumbs::LogLevel::Verbose,
                "guest-abi-calls",
                "timesinceboot called"
            );
            // timer interrupt occurs 200 times per second
            let time = *NUMBER_OF_TIMER_INTERRUPTS.lock() * 50_000;
            Ok(time as i32)
        }),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    match imports.define(
        "env",
        "cputime",
        Extern::typed_func(|_: FuncContext<'_>, _: ()| {
            log!(
                breadcrumbs::LogLevel::Verbose,
                "guest-abi-calls",
                "cputime called"
            );
            // timer interrupt occurs 200 times per second
            let time = *NUMBER_OF_TIMER_INTERRUPTS_SINCE_RESET.lock() * 50_000;
            Ok(time as i32)
        }),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    match imports.define(
        "env",
        "malloc",
        Extern::typed_func(|_: FuncContext<'_>, size: i32| {
            log!(
                breadcrumbs::LogLevel::Verbose,
                "guest-abi-calls",
                format!("malloc called with size: {}", size)
            );
            let mut memory_allocations = MEMORY_ALLOCATIONS.lock();
            match malloc((*memory_allocations.clone()).to_vec(), size as usize) {
                Ok(ptr) => {
                    *memory_allocations = ptr.0;
                    Ok(ptr.1 as i32)
                }
                Err(_) => Ok(-1),
            }
        }),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    };

    match imports.define(
        "env",
        "free",
        Extern::typed_func(|_: FuncContext<'_>, ptr: i32| {
            log!(
                breadcrumbs::LogLevel::Verbose,
                "guest-abi-calls",
                format!("free called with pointer: {}", ptr)
            );
            let mut memory_allocations = MEMORY_ALLOCATIONS.lock();
            match free((*memory_allocations.clone()).to_vec(), ptr as usize) {
                Ok(ptr) => {
                    *memory_allocations = ptr;
                    Ok(())
                }
                Err(_) => Ok(()),
            }
        }),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    };

    match imports.define(
        "env",
        "ptrsize",
        Extern::typed_func(|_: FuncContext<'_>, ptr: i32| {
            log!(
                breadcrumbs::LogLevel::Verbose,
                "guest-abi-calls",
                format!("ptrsize called with pointer: {}", ptr)
            );
            let memory_allocations = MEMORY_ALLOCATIONS.lock();
            for allocation in memory_allocations.iter() {
                if allocation.ptr == ptr as usize {
                    return Ok(allocation.size as i32);
                }
            }
            Ok(-1)
        }),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    match imports.define(
        "env",
        "memmove",
        Extern::typed_func(|mut ctx: FuncContext<'_>, data: (i32, i32, i32)| {
            log!(
                breadcrumbs::LogLevel::Verbose,
                "guest-abi-calls",
                format!("memmove called with data: {:?}", data)
            );
            let mut memory = match ctx.exported_memory_mut("memory") {
                Ok(memory) => memory,
                Err(_) => return Ok(-1),
            };
            let src = data.0 as usize;
            let dest = data.1 as usize;
            let size = data.2 as usize;
            let src_data;
            let dest_data;
            src_data = match memory.load(src, size) {
                Ok(d) => d.to_vec(),
                Err(_) => return Ok(-1),
            };
            dest_data = match memory.load(dest, size) {
                Ok(d) => d.to_vec(),
                Err(_) => return Ok(-1),
            };
            match memory.store(src, size, &dest_data) {
                Ok(_) => {}
                Err(_) => return Ok(-1),
            }
            match memory.store(dest, size, &src_data) {
                Ok(_) => {}
                Err(_) => return Ok(-1),
            }
            Ok(0)
        }),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    // memset
    match imports.define(
        "env",
        "memset",
        Extern::typed_func(|mut ctx: FuncContext<'_>, data: (i32, i32, i32)| {
            log!(
                breadcrumbs::LogLevel::Verbose,
                "guest-abi-calls",
                format!("memset called with data: {:?}", data)
            );
            let mut memory = match ctx.exported_memory_mut("memory") {
                Ok(memory) => memory,
                Err(_) => return Ok(-1),
            };
            let dest = data.0 as usize;
            let size = data.1 as usize;
            let value = data.2 as u8;
            let data = vec![value; size];
            match memory.store(dest, size, &data) {
                Ok(_) => Ok(0),
                Err(_) => Ok(-1),
            }
        }),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    // memcpy
    match imports.define(
        "env",
        "memcpy",
        Extern::typed_func(|mut ctx: FuncContext<'_>, data: (i32, i32, i32)| {
            log!(
                breadcrumbs::LogLevel::Verbose,
                "guest-abi-calls",
                format!("memcpy called with data: {:?}", data)
            );
            let mut memory = match ctx.exported_memory_mut("memory") {
                Ok(memory) => memory,
                Err(_) => return Ok(-1),
            };
            let src = data.0 as usize;
            let dest = data.1 as usize;
            let size = data.2 as usize;
            let data = match memory.load(src, size) {
                Ok(d) => d.to_vec(),
                Err(_) => return Ok(-1),
            };
            match memory.store(dest, size, &data) {
                Ok(_) => Ok(0),
                Err(_) => Ok(-1),
            }
        }),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    match imports.define(
        "env",
        "seedrng",
        Extern::typed_func(|mut ctx: FuncContext<'_>, data: i32| {
            log!(
                breadcrumbs::LogLevel::Verbose,
                "guest-abi-calls",
                format!("seedrng called with data: {}", data)
            );
            // the rng is seeded using a u64, but with the wasm interface we can only pass i32
            // so the i32 is a pointer to a u64 stored in the memory
            let memory = match ctx.exported_memory("memory") {
                Ok(memory) => memory,
                Err(_) => return Ok(-1),
            };
            // the length of a u64
            let size = 8;
            let seed = match memory.load(data as usize, size) {
                Ok(seed) => seed,
                Err(_) => return Ok(-1),
            };

            // turn &[u8] into a u64
            let seed = match seed.try_into() {
                Ok(seed) => u64::from_le_bytes(seed),
                Err(_) => return Ok(-1),
            };

            *RNG.lock() = rand::rngs::SmallRng::seed_from_u64(seed);
            Ok(0)
        }),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    match imports.define(
        "env",
        "rng",
        Extern::typed_func(|_: FuncContext<'_>, _: ()| {
            log!(
                breadcrumbs::LogLevel::Verbose,
                "guest-abi-calls",
                "rng called"
            );
            let result = (RNG.lock().next_u32() & 0x7FFFFFFF) as i32;
            Ok(result)
        }),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    // instantiating the module will run the start function
    let instance = match module.instantiate(&mut store, Some(imports)) {
        Ok(instance) => instance,
        Err(err) => {
            // reset memory allocations
            let mut memory_allocations = MEMORY_ALLOCATIONS.lock();
            *memory_allocations = Vec::new();
            return Err(err.to_string());
        }
    };

    // check if there's a function called "main" exported
    match instance.exported_func::<(), ()>(&mut store, "main") {
        Ok(func) => {
            // there's an exported function called "main", that means that what the program wants to run isn't in the start function
            match func.call(&mut store, ()) {
                Ok(_) => {}
                Err(err) => {
                    // reset memory allocations
                    let mut memory_allocations = MEMORY_ALLOCATIONS.lock();
                    *memory_allocations = Vec::new();
                    return Err(err.to_string());
                }
            }
        }
        Err(_) => {}
    }

    // reset memory allocations
    let mut memory_allocations = MEMORY_ALLOCATIONS.lock();
    *memory_allocations = Vec::new();

    Ok(())
}
