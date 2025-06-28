use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_apps.rs");
    let mut f = File::create(&dest_path).unwrap();

    let apps_dir = Path::new("./rootfs/Applications");
    let entries = fs::read_dir(apps_dir).unwrap();

    writeln!(f, "{{").unwrap(); // Write the opening brace

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    let app_name = file_name_str.trim_end_matches(".wasm");
                    let abs_path = fs::canonicalize(&path).unwrap();
                    writeln!(
                        f,
                        "register_app(\"{}\", include_bytes!(\"{}\"));",
                        app_name,
                        abs_path.display()
                    )
                    .unwrap();
                }
            }
        }
    }

    writeln!(f, "}}").unwrap();
}
