use crate::{error::Error, state::State};
use bonsai::subtree_index_to_general;
use interface::{address::ADDRESS_SIZE, Address};
use oof::Oof;

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

impl From<oof::Error> for Error<Address> {
    fn from(e: oof::Error) -> Self {
        match e {
            // TODO: calculate address cone a key is resides in
            oof::Error::EntryNotFound(_) => Error::MissingProof(Address::zero()),
        }
    }
}
