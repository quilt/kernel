use engine::Engine;
use ewasm::{Execute, RootRuntime};
use interface::{Contract, Transaction};
use wallet::Wallet;

#[ignore]
#[test]
fn test() {
    let wallet = Wallet::new();
    let mut runtime = RootRuntime::new(&wallet.asm(), &[], [0u8; 32]);

    runtime.set_logger(|b| {
        println!("{}", b);
    });

    let _ = runtime.execute();
}

#[test]
fn engine() {
    let mut engine = Engine::new(4);
    let wallet = Wallet::new();

    let address = engine.deploy(wallet);
    let tx = Transaction::new(address, vec![]);

    engine.execute(vec![tx]);
}
