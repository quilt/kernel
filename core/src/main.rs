#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

mod error;
mod process;
mod state;

#[cfg(not(test))]
mod host;

use arrayref::array_ref;
use state::{Oof, State};

pub const TREE_HEIGHT: u32 = 2;

#[cfg(test)]
pub extern "C" fn main() {}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn main() {
    let input_size = unsafe { host::eth2_blockDataSize() as usize };
    let mut input = [0u8; 42000];
    unsafe {
        host::eth2_blockDataCopy(input.as_mut_ptr() as *mut u32, 0, input_size as u32);
    }
    let mut pre_state_root = [0u8; 32];
    unsafe { host::eth2_loadPreStateRoot(pre_state_root.as_mut_ptr() as *const u32) }
    let post_root = process_data_blob(&mut input, &pre_state_root);
    unsafe { host::eth2_savePostStateRoot(post_root.as_ptr() as *const u32) }
}

#[cfg(not(test))]
fn process_data_blob(blob: &mut [u8], pre_state_root: &[u8; 32]) -> [u8; 32] {
    let (mem_offset, blob) = blob.split_at_mut(4);
    let mem_offset = u32::from_le_bytes(*array_ref!(mem_offset, 0, 4)) as usize;
    let (transactions, blob) = blob.split_at_mut(mem_offset);
    let mut mem = unsafe { Oof::from_raw(blob.as_mut_ptr(), 2) };
    let pre_root = mem.root().unwrap();
    assert_eq!(pre_state_root, pre_root);
    process::process_raw_transactions(&mut mem, transactions).expect("to process all transactions");
    *mem.root().unwrap()
}
