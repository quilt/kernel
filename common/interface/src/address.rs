use arrayref::array_ref;
use core::convert::{From, TryFrom};

pub const ADDRESS_SIZE: usize = 1;
type T = [u8; ADDRESS_SIZE];

#[derive(Clone, Copy, Debug)]
pub struct Address(T);

impl Address {
    pub fn new(t: T) -> Self {
        Address(t)
    }

    pub fn zero() -> Self {
        Address([0u8; ADDRESS_SIZE])
    }
}

const FIRST_ADDRESS: u128 = 1 << ((ADDRESS_SIZE * 8) as u32);
const LAST_ADDRESS: u128 = 1 << (((ADDRESS_SIZE + 1) * 8) as u32);

impl TryFrom<u128> for Address {
    type Error = u8;

    fn try_from(k: u128) -> Result<Self, Self::Error> {
        if k >= FIRST_ADDRESS && k <= LAST_ADDRESS {
            let bytes = k.to_le_bytes();
            Ok(Address::new(*array_ref![bytes, 0, ADDRESS_SIZE]))
        } else {
            Err(0)
        }
    }
}

impl From<Address> for u128 {
    fn from(a: Address) -> Self {
        let mut buf = [0u8; 16];
        buf.copy_from_slice(&a.0);
        FIRST_ADDRESS + u128::from_le_bytes(buf)
    }
}
