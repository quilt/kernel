use crate::address::Address;

pub trait Contract {
    fn new() -> Self;
    fn asm(&self) -> Vec<u8>;
    fn address(&self) -> Option<Address>;
}
