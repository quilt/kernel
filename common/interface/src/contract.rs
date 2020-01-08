use crate::address::Address;
use std::collections::BTreeMap;

pub trait Contract {
    fn new() -> Self;
    fn asm(&self) -> &[u8];
    fn to_map(&self) -> BTreeMap<u128, [u8; 32]>;
}
