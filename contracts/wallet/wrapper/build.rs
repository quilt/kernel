use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let status = Command::new(env::var("CARGO").unwrap())
        .arg("build")
        .arg("--target=wasm32-unknown-unknown")
        .arg("--manifest-path=../Cargo.toml")
        .arg(format!("--target-dir={}", out_dir))
        .status()
        .expect("failed to execute rustc");

    assert!(status.success());
}
