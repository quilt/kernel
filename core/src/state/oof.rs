use crate::{error::Error, state::State};
use alloc::vec::Vec;
use arrayref::array_ref;
use bonsai::{first_leaf, log2, next_power_of_two, subtree_index_to_general};
use core::convert::TryFrom;
use interface::Address;

type K = u128;
type V = [u8; 32];

const CODE_ROOT_INDEX: u128 = 6;
const CODE_LEN_INDEX: u128 = 7;

pub struct Oof {
    mem: oof::Oof,
    height: u32,
}

impl Oof {
    pub fn from_raw(data: *mut u8, height: u32) -> Self {
        Self {
            mem: unsafe { oof::Oof::from_raw(data) },
            height,
        }
    }

    pub fn address_root(&self, address: Address) -> u128 {
        first_leaf(1, self.height as u128) + u128::from(address)
    }

    pub fn get_bytes(&self, root: u128, len: u128) -> Result<Vec<u8>, Error<Address>> {
        let mut bytes = Vec::with_capacity(len as usize);
        let chunks: u128 = len / 32 + if len % 32 == 0 { 0 } else { 1 };
        let first_chunk = &first_leaf(root, log2(next_power_of_two(chunks)));

        for i in 0..chunks {
            let index = first_chunk + i;
            let value = self.mem.get(&index).ok_or(Error::MissingNode(index))?;

            if i == chunks - 1 && len % 32 != 0 {
                bytes.extend(&value[0..(len % 32) as usize]);
            } else {
                bytes.extend(value);
            }
        }

        Ok(bytes)
    }
}

impl State<K, V> for Oof {
    fn root(&mut self) -> Result<&V, Error<Address>> {
        self.mem.root().map_err(|e| e.into())
    }

    fn code(&self, address: &Address) -> Result<Vec<u8>, Error<Address>> {
        let root = self.address_root(*address);
        let index = subtree_index_to_general(root, CODE_LEN_INDEX);
        let raw_mixin = self.mem.get(&index).ok_or(Error::MissingNode(index))?;
        let len = u128::from_le_bytes(*array_ref![raw_mixin, 0, 16]);

        self.get_bytes(subtree_index_to_general(root, CODE_ROOT_INDEX), len)
    }

    fn deploy(&mut self, address: Address, code: &[u8]) -> Result<(), Error<Address>> {
        unimplemented!()
    }

    fn get(&self, address: &Address, key: &K) -> Option<&V> {
        let key = subtree_index_to_general((*address).into(), *key);
        self.mem.get(&key)
    }

    fn set(&mut self, address: &Address, key: K, value: V) -> Result<Option<V>, Error<Address>> {
        let key = subtree_index_to_general((*address).into(), key);
        Ok(self.mem.set(key, value))
    }
}

impl From<oof::Error> for Error<Address> {
    fn from(e: oof::Error) -> Self {
        match e {
            // TODO: calculate address cone a key is resides in
            oof::Error::EntryNotFound(_) => Error::MissingProof(Address::zero()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use interface::Contract;
    use std::collections::BTreeMap;
    use std::mem::size_of;
    use wallet::Wallet;

    #[test]
    fn real() {
        let mut blob = [
            8, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 89,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 245, 165, 253, 66, 209, 106, 32,
            48, 39, 152, 239, 110, 211, 9, 151, 155, 67, 0, 61, 35, 32, 217, 240, 232, 234, 152,
            49, 169, 39, 89, 251, 75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 95, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 115, 109, 1, 0, 0, 0, 1,
            9, 2, 96, 2, 127, 127, 0, 96, 0, 0, 2, 13, 1, 3, 101, 110, 118, 5, 112, 114, 105, 110,
            116, 0, 0, 3, 2, 1, 1, 5, 3, 1, 0, 1, 7, 17, 2, 6, 109, 101, 109, 111, 114, 121, 2, 0,
            4, 109, 97, 105, 110, 0, 1, 10, 11, 1, 9, 0, 65, 232, 7, 65, 11, 16, 0, 11, 11, 18, 1,
            0, 65, 232, 7, 11, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0,
        ];

        let mut oof = unsafe { Oof::from_raw(blob.as_mut_ptr(), 2) };
        println!("{:?}", oof.map.keys());
        oof.root().unwrap();
        println!("{:?}", oof.code(&Address::one()));
    }
}
