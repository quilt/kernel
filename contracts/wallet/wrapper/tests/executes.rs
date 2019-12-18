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
    let mut engine = Engine::new();
    let mut wallet = Wallet::new();

    engine.deploy(&mut wallet);
    let tx = Transaction::new(wallet.address().unwrap(), vec![]);

    engine.execute(vec![tx]);
}
