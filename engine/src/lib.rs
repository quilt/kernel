use interface::{Address, Contract};

struct DeployedContract<'a, T: Contract> {
    contract: &'a T,
    address: Option<Address>,
}

impl<'a, T: Contract> DeployedContract<'a, T> {
    pub fn new(contract: &'a T) -> Self {
        Self {
            contract,
            address: None,
        }
    }
}

struct Engine<'a, T: Contract> {
    contracts: Vec<DeployedContract<'a, T>>,
}

impl<'a, T: Contract> Engine<'a, T> {
    pub fn new() -> Self {
        Self { contracts: vec![] }
    }

    fn asm(&self) -> Vec<u8> {
        let ret = include_bytes!(concat!(
            env!("OUT_DIR"),
            "/wasm32-unknown-unknown/debug/core.wasm"
        ));
        ret.to_vec()
    }

    pub fn deploy(&mut self, contract: &'a T) {
        self.contracts.push(DeployedContract::new(contract));
    }

    pub fn execute(&self) {
        unimplemented!()
    }
}
