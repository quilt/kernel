use interface::{Address, Contract};

pub struct Wallet {
    address: Option<Address>,
}

impl Contract for Wallet {
    fn new() -> Self {
        Self { address: None }
    }

    fn asm(&self) -> Vec<u8> {
        let ret = include_bytes!(concat!(
            env!("OUT_DIR"),
            "/wasm32-unknown-unknown/debug/wallet-backend.wasm"
        ));
        ret.to_vec()
    }

    fn address(&self) -> Option<Address> {
        self.address
    }
}
