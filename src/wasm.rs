use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use tinywasm::{Extern, FuncContext, MemoryStringExt};

use crate::{get_current_byte_in_stdin, print, println, reset_allowed_backspaces, serial_print};

pub fn run_from_bytes(args: String, bytes: &[u8]) -> Result<(), String> {
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
            match memory.store(0, args_len, args) {
                Ok(_) => return Ok(0),
                Err(_) => return Ok(-1),
            }
        }),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    // instantiating the module will run the start function
    let instance = match module.instantiate(&mut store, Some(imports)) {
        Ok(instance) => instance,
        Err(err) => return Err(err.to_string()),
    };

    // check if there's a function called "main" exported
    match instance.exported_func::<(), ()>(&mut store, "main") {
        Ok(func) => {
            // there's an exported function called "main", that means that what the program wants to run isn't in the start function
            match func.call(&mut store, ()) {
                Ok(_) => {}
                Err(err) => return Err(err.to_string()),
            }
        }
        Err(_) => {}
    }

    Ok(())
}
