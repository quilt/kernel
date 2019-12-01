use bonsai::{log2, subtree_index_to_general};
use interface::{Address, Contract};
use std::{cmp::min, collections::HashMap, mem::transmute};

#[derive(Clone)]
pub struct Wallet {
    address: Option<Address>,
    asm: Vec<u8>,
}

impl Contract for Wallet {
    fn new() -> Self {
        Self {
            address: None,
            asm: include_bytes!(concat!(
                env!("OUT_DIR"),
                "/wasm32-unknown-unknown/debug/wallet-backend.wasm"
            ))
            .to_vec(),
        }
    }

    fn asm(&self) -> &[u8] {
        &self.asm
    }

    fn to_map(&self) -> HashMap<u128, [u8; 32]> {
        let asm = self.asm();
        let len = asm.len() as u128;
        let padded_len = (asm.len() as u128)
            .checked_next_power_of_two()
            .expect("compiled code to fit in tree");
        let depth = log2(padded_len / 32);
        let first: u128 = subtree_index_to_general(6, 1 << depth - 1);

        let mut map = HashMap::<u128, [u8; 32]>::new();

        // insert length mix-in
        map.insert(7, u32_to_value(len as u32));

        // insert code chunks by index
        for i in (0..len).step_by(32) {
            let mut buf = [0u8; 32];

            let begin = i as usize;
            let end = min(i + 32, len) as usize;

            buf[0..(if end % 32 != 0 { end % 32 } else { 32 })].copy_from_slice(&asm[begin..end]);

            map.insert(first + (i / 32), buf);
        }

        // insert padding chunks by index
        for i in (len..padded_len).step_by(32) {
            map.insert(first + (i / 32), [0u8; 32]);
        }

        map
    }

    fn address(&self) -> Option<Address> {
        self.address
    }

    fn set_address(&mut self, address: Address) {
        self.address = Some(address);
    }
}

fn u32_to_value(n: u32) -> [u8; 32] {
    let mut buf = [0u32; 8];
    buf[0] = n;
    unsafe { transmute::<[u32; 8], [u8; 32]>(buf) }
}

#[cfg(test)]
mod test {
    use super::*;
    use arrayref::array_ref;

    fn build_value(n: &u32) -> Vec<u8> {
        u32_to_value(*n).to_vec()
    }

    #[test]
    fn to_map() {
        let wallet = Wallet {
            address: None,
            asm: vec![1u32, 2, 3, 4, 5, 6]
                .iter()
                .flat_map(build_value)
                .collect::<Vec<u8>>(),
        };

        let mut map = HashMap::<u128, [u8; 32]>::new();
        map.insert(7, u32_to_value(6 * 32));

        for i in (0..wallet.asm.len()).step_by(32) {
            map.insert((24 + (i / 32)) as u128, *array_ref![wallet.asm, i, 32]);
        }

        map.insert(30, [0u8; 32]);
        map.insert(31, [0u8; 32]);

        assert_eq!(wallet.to_map(), map);
    }
}
