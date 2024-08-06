use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use spin::Mutex;
use tinywasm::{Extern, FuncContext, MemoryStringExt};

use crate::{
    get_current_byte_in_stdin, interrupts::{NUMBER_OF_TIMER_INTERRUPTS, NUMBER_OF_TIMER_INTERRUPTS_SINCE_RESET}, reset_allowed_backspaces, serial_print, serial_println
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
        if allocation.ptr - ptr >= size {
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
    serial_println!("running from bytes");
    static MEMORY_ALLOCATIONS: Mutex<Vec<Allocation>> = Mutex::new(Vec::new());
    serial_println!("55");
    let module = match tinywasm::Module::parse_bytes(bytes) {
        Ok(module) => module,
        Err(err) => return Err(err.to_string()),
    };
    serial_println!("60");
    let mut store = tinywasm::Store::default();
    serial_println!("62");
    let mut imports = tinywasm::Imports::new();

    serial_println!("Starting defining imports");

    match imports.define(
        "env",
        "putchar",
        Extern::typed_func(|_: FuncContext<'_>, v: i32| {
            serial_print!("{}", v as u8 as char);
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
            serial_println!("abort called");
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

    serial_println!("Finished defining imports");

    // instantiating the module will run the start function
    let instance = match tinywasm::ModuleInstance::instantiate(&mut store, module, Some(imports)) {
        Ok(instance) => instance,
        Err(err) => return Err(err.to_string()),
    };

    serial_println!("Finished instantiating module");

    // check if there's a function called "main" exported
    match instance.exported_func::<(), ()>(&mut store, "main") {
        Ok(func) => {
            // there's an exported function called "main", that means that what the program wants to run isn't in the start function
            match func.call(&mut store, ()) {
                Ok(_) => {}
                Err(err) => return Err(err.to_string()),
            }
        }
        Err(_) => {
            // run the normal start function
            match instance.start(&mut store) {
                Ok(_) => {}
                Err(err) => return Err(err.to_string()),
            }
        }
    }

    Ok(())
}
