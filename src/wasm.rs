use core::convert::TryInto;

use alloc::{
    string::{String, ToString},
    vec::Vec,
    vec
};
use lazy_static::lazy_static;
use rand::{RngCore, SeedableRng};
use spin::Mutex;
use tinywasm::{Extern, FuncContext, MemoryStringExt};

use crate::{
    get_current_byte_in_stdin,
    interrupts::{NUMBER_OF_TIMER_INTERRUPTS, NUMBER_OF_TIMER_INTERRUPTS_SINCE_RESET},
    print, println, reset_allowed_backspaces, serial_print
};

#[derive(Clone, Copy)]
struct Allocation {
    ptr: usize,
    size: usize,
}

#[derive(Clone)]
struct AllocCommandReturn(Vec<Allocation>, usize);

fn malloc(allocations: Vec<Allocation>, size: usize) -> Result<AllocCommandReturn, String> {
    // try and find room
    let mut ptr = 0;
    for allocation in allocations.iter() {
        if allocation.ptr >= ptr && allocation.ptr - ptr >= size {
            break;
        }
        ptr = allocation.ptr + allocation.size;
    }
    // if we didn't find room, allocate at the end
    if ptr == 0 {
        if let Some(last) = allocations.last() {
            ptr = last.ptr + last.size;
        }
    }
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
    static MEMORY_ALLOCATIONS: Mutex<Vec<Allocation>> = Mutex::new(Vec::new());
    lazy_static! {
        static ref RNG: Mutex<rand::rngs::SmallRng> =
            Mutex::new(rand::rngs::SmallRng::seed_from_u64(1u64));
    }
    let module = match tinywasm::Module::parse_bytes(bytes) {
        Ok(module) => module,
        Err(err) => return Err(err.to_string()),
    };
    let mut store = tinywasm::Store::default();
    let mut imports = tinywasm::Imports::new();

    match imports.define(
        "env",
        "putchar",
        Extern::typed_func(|_: FuncContext<'_>, v: i32| {
            print!("{}", v as u8 as char);
            reset_allowed_backspaces();
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
            serial_print!("{}", v as u8 as char);
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
            let byte = get_current_byte_in_stdin();
            Ok(byte as i32)
        }),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    match imports.define(
        "env",
        "abort",
        Extern::typed_func(|_: FuncContext<'_>, _: (i32, i32, i32, i32)| {
            println!("abort called");
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
