use arborist::Tree;
use bonsai::{log2, subtree_index_to_general};
use ewasm::{Execute, RootRuntime};
use interface::{Address, Contract, Transaction};
use oof::Oof;
use std::collections::BTreeMap;
use std::mem::size_of;

pub struct Engine<T: Contract> {
    contracts: Vec<(Address, T)>,
    max_address: u128,
}

impl<T: Contract> Engine<T> {
    pub fn new(max_address: u128) -> Self {
        assert!(log2(max_address) < (size_of::<Address>() * 8) as u128);

        Self {
            contracts: vec![],
            max_address,
        }
    }

    fn asm(&self) -> Vec<u8> {
        let ret = include_bytes!(concat!(
            env!("OUT_DIR"),
            "/wasm32-unknown-unknown/debug/core.wasm"
        ));
        ret.to_vec()
    }

    pub fn deploy(&mut self, contract: T) -> Address {
        // TODO: generate random address to deploy to, or accept address as paramater
        let address = Address::zero();
        self.contracts.push((address, contract));
        address
    }

    pub fn execute(&self, txs: Vec<Transaction>) {
        let (blob, root) = self.blob(txs);
        let mut runtime = RootRuntime::new(&self.asm(), &blob, root);

        runtime.set_logger(|s| println!("{}", s));

        println!("post {:?}", runtime.execute());
    }

    fn blob(&self, txs: Vec<Transaction>) -> (Vec<u8>, [u8; 32]) {
        let tx_len: u32 = txs.iter().map(|tx| tx.len()).sum::<u32>() + 4;
        let txs: Vec<u8> = txs.iter().flat_map(|tx| tx.to_bytes()).collect();

        let mut data = tx_len.to_le_bytes().to_vec();
        data.extend(txs);
        data.extend(self.proof().to_bytes());

        (data, *self.proof().root().expect("is valid"))
    }

    fn proof(&self) -> Oof {
        let mut mem = BTreeMap::<u128, [u8; 32]>::new();
        let first = 1 << log2(self.max_address as u128);

        for (a, c) in self.contracts.iter() {
            let storage = c.to_map();
            let index: u128 = first + u128::from(*a);

            // fill in the contracts proof
            let mut tree = Tree::from_map(storage);
            tree.fill_subtree(1, 1, &[0; 32]);

            let remapped: BTreeMap<u128, [u8; 32]> = tree
                .to_map()
                .iter()
                .map(|(k, v)| (subtree_index_to_general(index, *k), *v))
                .collect();

            mem.extend(remapped);
        }

        let mut tree = Tree::from_map(mem);
        tree.fill_subtree(1, 2, &[0u8; 32]);
        let ret = Oof::from_map(tree.trim().to_map());

        println!("{:?}", ret.keys());
        println!("{:?}", ret.to_bytes());

        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub struct MockContract {
        storage: BTreeMap<u128, [u8; 32]>,
    }

    impl MockContract {
        fn with_storage(storage: BTreeMap<u128, [u8; 32]>) -> Self {
            Self { storage }
        }
    }

    impl Contract for MockContract {
        fn new() -> Self {
            todo!()
        }

        fn asm(&self) -> &[u8] {
            &[1, 2, 3, 4]
        }

        fn to_map(&self) -> BTreeMap<u128, [u8; 32]> {
            self.storage.clone()
        }
    }

    #[test]
    fn state_proof() {
        let mut engine = Engine::new(4);

        let mut storage = BTreeMap::new();
        storage.insert(2, [2; 32]);
        storage.insert(3, [3; 32]);
        let contract = MockContract::with_storage(storage);

        let address = engine.deploy(contract);
        let mut oof = engine.proof();

        println!("address: {:?}", address);
        println!("{:?}", oof.into_branch().to_map());
    }
}
