use crate::address::Address;
use arrayref::array_ref;
use core::{mem::size_of, slice};

#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct Transaction {
    ptr: *const u8,
    length: u32,
}

impl Transaction {
    pub fn from_ptr(ptr: *const u8) -> Transaction {
        let length =
            u32::from_le_bytes(*array_ref![unsafe { slice::from_raw_parts(ptr, 4) }, 0, 4]);

        Transaction { ptr, length }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn to(&self) -> &Address {
        unsafe {
            core::mem::transmute::<&[u8; size_of::<Address>()], &Address>(array_ref![
                slice::from_raw_parts(self.ptr.offset(4), size_of::<Address>()),
                0,
                size_of::<Address>()
            ])
        }
    }

    pub fn data(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self.ptr.offset(4 + size_of::<Address>() as isize),
                self.length as usize - size_of::<Address>(),
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_call() {
        let length = 143u32;
        let to = [0u8; 20];
        let data = [3u8; 123];

        let mut raw_tx = [0u8; 4 + 20 + 123];
        raw_tx[0..4].copy_from_slice(&length.to_le_bytes());
        raw_tx[4..24].copy_from_slice(&to);
        raw_tx[24..147].copy_from_slice(&data);

        let tx = Transaction::from_ptr(raw_tx.as_ptr());

        assert_eq!(tx.length(), length);
        assert_eq!(tx.to(), &to);
        assert_eq!(&tx.data()[..], &data[..]);
    }
}
