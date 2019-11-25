use crate::error::Error;
use crate::state::State;
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

type A = u128;
type K = u128;
type V = u128;

#[derive(Hash)]
pub struct MockAccount {
    code: Vec<u8>,
    storage: BTreeMap<K, V>,
}

pub struct MockState {
    accounts: BTreeMap<A, MockAccount>,
}

impl MockState {
    pub fn new(accounts: BTreeMap<A, MockAccount>) -> MockState {
        MockState { accounts }
    }
}

impl State<A, K, V> for MockState {
    fn root(&mut self) -> Result<V, Error<A>> {
        let mut s = DefaultHasher::new();
        self.accounts.hash(&mut s);
        let hash = V::from(s.finish());
        Ok(hash.into())
    }

    fn code(&self, address: &A) -> Result<&[u8], Error<A>> {
        match self.accounts.get(address) {
            Some(account) => Ok(&account.code),
            None => Err(Error::MissingProof(*address)),
        }
    }

    fn deploy(&mut self, address: A, code: &[u8]) -> Result<(), Error<A>> {
        let account = MockAccount {
            code: code.to_vec(),
            storage: BTreeMap::<K, V>::default(),
        };

        match self.accounts.insert(address, account) {
            Some(_) => Ok(()),
            None => Err(Error::MissingProof(address)),
        }
    }

    fn get(&self, address: &A, key: &K) -> Option<&V> {
        match self.accounts.get(address) {
            Some(account) => account.storage.get(key),
            None => None,
        }
    }

    fn set(&mut self, address: &A, key: K, value: V) -> Result<Option<V>, Error<A>> {
        match self.accounts.get_mut(address) {
            Some(account) => Ok(account.storage.insert(key, value)),
            None => Err(Error::MissingProof(*address)),
        }
    }
}
