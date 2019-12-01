use crate::{address::ADDRESS_SIZE, Address};
use arrayref::array_ref;
use core::{mem::size_of, slice};

#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct RawTransaction {
    ptr: *const u8,
    length: u32,
}

impl RawTransaction {
    pub fn from_ptr(ptr: *const u8) -> Self {
        let length =
            u32::from_le_bytes(*array_ref![unsafe { slice::from_raw_parts(ptr, 4) }, 0, 4]);

        Self { ptr, length }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn to(&self) -> &Address {
        unsafe {
            core::mem::transmute::<&[u8; ADDRESS_SIZE], &Address>(array_ref![
                slice::from_raw_parts(self.ptr.offset(4), ADDRESS_SIZE),
                0,
                size_of::<Address>()
            ])
        }
    }

    pub fn data(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self.ptr.offset(4 + ADDRESS_SIZE as isize),
                self.length as usize - ADDRESS_SIZE,
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_call() {
        let length = ADDRESS_SIZE as u32 + 123u32;
        let to = [0u8; ADDRESS_SIZE];
        let data = [3u8; 123];

        let mut raw_tx = [0u8; 4 + ADDRESS_SIZE + 123];
        raw_tx[0..4].copy_from_slice(&length.to_le_bytes());
        raw_tx[4..(4 + ADDRESS_SIZE)].copy_from_slice(&to);
        raw_tx[(4 + ADDRESS_SIZE)..(4 + ADDRESS_SIZE + 123)].copy_from_slice(&data);

        let tx = RawTransaction::from_ptr(raw_tx.as_ptr());

        assert_eq!(tx.length(), length);
        assert_eq!(tx.to(), &Address::new(to));
        assert_eq!(&tx.data()[..], &data[..]);
    }
}
