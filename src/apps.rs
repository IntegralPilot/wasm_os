use crate::{println, wasm::run_from_bytes};
use alloc::{
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use lazy_static::lazy_static;
use spin::Mutex;

#[derive(Clone)]
struct App {
    name: String,
    wasm_bytes: Vec<u8>,
}

lazy_static! {
    static ref REGISTERED_APPS: Arc<Mutex<Vec<App>>> = Arc::new(Mutex::new(Vec::new()));
}

pub fn register_app(name: &str, wasm_bytes: &[u8]) {
    REGISTERED_APPS.lock().push(App {
        name: name.to_string(),
        wasm_bytes: wasm_bytes.to_vec(),
    });
}

pub fn run_app(command_line: &str) -> Result<(), i32> {
    //println!("Running app: {}", command_line);
    let app_name = command_line.split_whitespace().next().unwrap_or("");
    let apps;
    {
        let temp_apps = REGISTERED_APPS.lock();
        apps = temp_apps.clone();
    }
    // only capture everything before the first space
    // i.e. "app_name arg1 arg2" -> "app_name"
    let app = apps.iter().find(|app| app.name == app_name);
    let app = match app {
        Some(app) => app,
        None => return Err(1),
    };
    match run_from_bytes(command_line.to_string(), &app.wasm_bytes) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Error running app: {}", e);
            Err(2)
        }
    }
}
