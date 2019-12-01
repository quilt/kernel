use crate::Address;
use std::mem::size_of;

pub struct Transaction {
    to: Address,
    data: Vec<u8>,
}

impl Transaction {
    pub fn new(to: Address, data: Vec<u8>) -> Self {
        Self { to, data }
    }

    pub fn len(&self) -> u32 {
        (size_of::<Address>() + self.data.len()) as u32
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut ret = self.len().to_le_bytes().to_vec();
        ret.extend(self.to.as_bytes().to_vec());
        ret.extend(self.data.clone());
        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::RawTransaction;

    #[test]
    fn to_bytes() {
        let tx = Transaction::new(Address::one(), vec![2; 100]);
        let bytes = tx.to_bytes();
        let raw = RawTransaction::from_ptr(bytes.as_ptr());

        assert_eq!(&tx.to, raw.to());
        assert_eq!(&tx.data[..], &raw.data()[..]);
    }
}
