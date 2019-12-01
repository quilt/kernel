use bonsai::subtree_index_to_general;
use ewasm::{Execute, RootRuntime};
use interface::{Address, Contract, Transaction};
use oof::Oof;
use std::collections::HashMap;
use std::mem::size_of;

pub struct Engine<T: Contract + Clone> {
    contracts: Vec<T>,
}

impl<T: Contract + Clone> Engine<T> {
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

    pub fn deploy(&mut self, contract: &mut T) {
        contract.set_address(Address::one());
        self.contracts.push(contract.clone());
    }

    pub fn execute(&self, transactions: Vec<Transaction>) {
        let mut mem = HashMap::<u128, [u8; 32]>::new();
        let first = 1 << (size_of::<Address>() * 8 - 1);

        for c in self.contracts.iter() {
            let storage = c.to_map();
            storage
                .iter()
                .map(|(k, v)| (subtree_index_to_general(first, *k), v));
            mem.extend(storage);
        }

        let tx_len: u32 = transactions.iter().map(|tx| tx.len()).sum::<u32>() + 4;
        let keys: Vec<u8> = mem.keys().flat_map(|k| k.to_le_bytes().to_vec()).collect();
        let values: Vec<u8> = mem.values().flat_map(|v| v.to_vec()).collect::<Vec<u8>>();
        let transactions: Vec<u8> = transactions.iter().flat_map(|tx| tx.to_bytes()).collect();

        let mut data = tx_len.to_le_bytes().to_vec();
        data.extend(transactions);
        data.extend(keys);
        data.extend(values);

        let mut runtime = RootRuntime::new(&self.asm(), &data, [0u8; 32]);
        let post = runtime.execute();
    }
}
