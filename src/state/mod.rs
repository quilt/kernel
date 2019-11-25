#[cfg(test)]
mod mock;

use crate::error::Error;

pub trait State<A, K, V> {
    fn root(&mut self) -> Result<V, Error<A>>;
    fn code(&self, address: &A) -> Result<&[u8], Error<A>>;
    fn deploy(&mut self, address: A, code: &[u8]) -> Result<(), Error<A>>;
    fn get(&self, address: &A, key: &K) -> Option<&V>;
    fn set(&mut self, address: &A, key: K, value: V) -> Result<Option<V>, Error<A>>;
}
