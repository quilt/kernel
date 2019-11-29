use ewasm::{Execute, RootRuntime};
use wallet::asm;

#[test]
fn test() {
    let mut runtime = RootRuntime::new(&asm(), &[], [0u8; 32]);

    runtime.set_logger(|b| {
        println!("{}", b);
    });

    let _ = runtime.execute();
}
