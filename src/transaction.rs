use arrayref::array_ref;
use core::slice;

pub type Address = [u8; 20];

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Transaction {
    Deploy(Deploy),
    Call(Call),
}

impl Transaction {
    pub fn from_ptr(ptr: *const u8) -> Transaction {
        let tx_type =
            u32::from_le_bytes(*array_ref![unsafe { slice::from_raw_parts(ptr, 4) }, 0, 4]);

        match tx_type {
            0 => unimplemented!(),
            1 => Transaction::Call(Call::new(unsafe { ptr.offset(4) })),
            _ => unreachable!(),
        }
    }

    pub fn length(&self) -> u32 {
        match self {
            Transaction::Deploy(d) => unimplemented!(),
            Transaction::Call(c) => 144 + c.length(),
        }
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Deploy;

#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct Call {
    ptr: *const u8,
    data_length: u32,
}

impl Call {
    pub fn new(ptr: *const u8) -> Call {
        let data_length = u32::from_le_bytes(*array_ref![
            unsafe { slice::from_raw_parts(ptr.offset(144), 4) },
            0,
            4
        ]);

        #[cfg(test)]
        println!("{:?}", data_length);

        Call { ptr, data_length }
    }

    pub fn length(&self) -> u32 {
        148 + self.data_length
    }

    pub fn to(&self) -> &Address {
        array_ref![unsafe { slice::from_raw_parts(self.ptr, 20) }, 0, 20]
    }

    pub fn from(&self) -> &Address {
        array_ref![
            unsafe { slice::from_raw_parts(self.ptr.offset(20), 20) },
            0,
            20
        ]
    }

    pub fn nonce(&self) -> u64 {
        u64::from_le_bytes(*array_ref![
            unsafe { slice::from_raw_parts(self.ptr.offset(40), 8) },
            0,
            8
        ])
    }

    pub fn signature(&self) -> &[u8; 96] {
        array_ref![
            unsafe { slice::from_raw_parts(self.ptr.offset(48), 96) },
            0,
            96
        ]
    }

    pub fn data(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr.offset(148), self.data_length as usize) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_call() {
        let ty = 1u32;
        let to = [0u8; 20];
        let from = [1u8; 20];
        let nonce = 42u64;
        let signature = [2u8; 96];
        let data_length = 123u32;
        let data = [3u8; 123];

        let mut raw_call = [0u8; 4 + 32 + 32 + 8 + 96 + 123];
        raw_call[0..4].copy_from_slice(&ty.to_le_bytes());
        raw_call[4..24].copy_from_slice(&to);
        raw_call[24..44].copy_from_slice(&from);
        raw_call[44..52].copy_from_slice(&nonce.to_le_bytes());
        raw_call[52..148].copy_from_slice(&signature);
        raw_call[148..152].copy_from_slice(&data_length.to_le_bytes());
        raw_call[152..275].copy_from_slice(&data);

        let tx = Transaction::from_ptr(raw_call.as_ptr());
        let call = Call::new(raw_call[4..].as_ptr());

        assert_eq!(tx, Transaction::Call(call.clone()));
        assert_eq!(call.to(), &to);
        assert_eq!(call.from(), &from);
        assert_eq!(call.nonce(), nonce);
        assert_eq!(&call.signature()[..], &signature[..]);
        assert_eq!(call.length(), 148 + data_length);
        assert_eq!(&call.data()[..], &data[..]);
    }
}
