use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let binary_name = Path::new("main.rs");
    let input_path = Path::new(&manifest_dir).join("src").join(binary_name);

    Command::new(env::var("RUSTC").unwrap())
        .arg("--target=wasm32-unknown-unknown")
        .arg(format!("--out-dir={}", out_dir))
        .arg(format!("{}", input_path.to_string_lossy()))
        .spawn()
        .expect("rustc to execute");
}
