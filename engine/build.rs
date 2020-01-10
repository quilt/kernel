use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../core/none");

    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let binary_name = Path::new("main.rs");
    let input_path = Path::new(&manifest_dir).join("src").join(binary_name);

    let status = Command::new(env::var("CARGO").unwrap())
        .arg("build")
        .arg("--target=wasm32-unknown-unknown")
        .arg("--manifest-path=../core/Cargo.toml")
        .arg(format!("--target-dir={}", out_dir))
        .status()
        .expect("failed to execute rustc");

    assert!(status.success());
}
