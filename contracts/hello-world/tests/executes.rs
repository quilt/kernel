use engine::Engine;
use ewasm::{Execute, RootRuntime};
use hello_world::HelloWorld;
use interface::{Contract, Transaction};

#[ignore]
#[test]
fn test() {
    let wallet = HelloWorld::new();
    let mut runtime = RootRuntime::new(&wallet.asm(), &[], [0u8; 32]);

    runtime.set_logger(|b| {
        println!("{}", b);
    });

    println!("about to execute");
    let _ = runtime.execute();
}

#[test]
fn engine() {
    let mut engine = Engine::new(4);
    let hello = HelloWorld::new();

    let address = engine.deploy(hello);
    let tx = Transaction::new(address, vec![]);

    engine.execute(vec![tx]);
}
