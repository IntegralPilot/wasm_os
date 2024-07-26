use alloc::string::{String, ToString};
use blog_os::{get_current_byte_in_stdin, reset_allowed_backspaces, serial_print};
use blog_os::{print, println};
use tinywasm::{Extern, FuncContext};

pub fn run_from_bytes(bytes: &[u8]) -> Result<(), String> {
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
