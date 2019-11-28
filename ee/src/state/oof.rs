use crate::{
    address::{Address, ADDRESS_SIZE},
    error::Error,
    state::State,
};
use arrayref::array_ref;
use bonsai::subtree_index_to_general;
use core::convert::{From, TryFrom};
use oof::Oof;

const FIRST_ADDRESS: u128 = 1 << ((ADDRESS_SIZE * 8) as u32);
const LAST_ADDRESS: u128 = 1 << (((ADDRESS_SIZE + 1) * 8) as u32);

type K = u128;
type V = [u8; 32];

impl<'a> State<K, V> for Oof<'a> {
    fn root(&mut self) -> Result<&V, Error<Address>> {
        Oof::root(self).map_err(|e| e.into())
    }

    fn code(&self, _: &Address) -> Result<&[u8], Error<Address>> {
        unimplemented!()
    }

    fn deploy(&mut self, address: Address, code: &[u8]) -> Result<(), Error<Address>> {
        unimplemented!()
    }

    fn get(&self, address: &Address, key: &K) -> Option<&V> {
        let key = subtree_index_to_general((*address).into(), *key);
        Oof::get(self, &key)
    }

    fn set(&mut self, address: &Address, key: K, value: V) -> Result<Option<V>, Error<Address>> {
        let key = subtree_index_to_general((*address).into(), key);
        Oof::set(self, key, value)
            .map(|x| Some(x))
            .map_err(|e| e.into())
    }
}

impl TryFrom<K> for Address {
    type Error = u8;

    fn try_from(k: K) -> Result<Self, Self::Error> {
        if k >= FIRST_ADDRESS && k <= LAST_ADDRESS {
            let bytes = k.to_le_bytes();
            Ok(Address::new(*array_ref![bytes, 0, ADDRESS_SIZE]))
        } else {
            Err(0)
        }
    }
}

impl From<Address> for K {
    fn from(a: Address) -> Self {
        let mut buf = [0u8; 16];
        buf.copy_from_slice(&a.0);
        FIRST_ADDRESS + u128::from_le_bytes(buf)
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
