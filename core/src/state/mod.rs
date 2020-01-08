mod oof;
pub use crate::state::oof::Oof;

use crate::error::Error;
use alloc::vec::Vec;
use interface::Address;

pub trait State<K, V> {
    fn root(&mut self) -> Result<&V, Error<Address>>;
    fn code(&self, address: &Address) -> Result<Vec<u8>, Error<Address>>;
    fn deploy(&mut self, address: Address, code: &[u8]) -> Result<(), Error<Address>>;
    fn get(&self, address: &Address, key: &K) -> Option<&V>;
    fn set(&mut self, address: &Address, key: K, value: V) -> Result<Option<V>, Error<Address>>;
}
