use ewasm::{Execute, RootRuntime};
use interface::Contract;
use wallet::Wallet;

#[test]
fn test() {
    let wallet = Wallet::new();

    let mut runtime = RootRuntime::new(&wallet.asm(), &[], [0u8; 32]);

    runtime.set_logger(|b| {
        println!("{}", b);
    });

    let _ = runtime.execute();
}
