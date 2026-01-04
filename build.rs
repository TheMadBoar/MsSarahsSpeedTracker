use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    
    let mut target_dir = PathBuf::from(out_dir);
    target_dir.pop();
    target_dir.pop();
    target_dir.pop();
    
    let placeholder_path = target_dir.join("DROP SPEEDTEST EXE HERE.txt");
    fs::write(
        placeholder_path,
        "Place the speedtest executable in this directory.\n"
    ).expect("Failed to create placeholder file");
    
    println!("cargo:rerun-if-changed=build.rs");
}