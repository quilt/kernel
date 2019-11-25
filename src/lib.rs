#![cfg_attr(not(test), no_std)]

mod address;
mod error;
mod process;
mod state;
mod transaction;

use arrayref::array_ref;
use oof::Oof;
use process::process_raw_transactions;

pub(crate) mod host {
    extern "C" {
        // state root interface
        pub fn eth2_loadPreStateRoot(offset: *const u32);
        pub fn eth2_savePostStateRoot(offset: *const u32);

        // block interface
        pub fn eth2_blockDataSize() -> u32;
        pub fn eth2_blockDataCopy(outputOfset: *const u32, offset: u32, length: u32);

        // buffer interface
        pub fn eth_bufferGet(frame: u32, key_ptr: *const u32, value_ptr: *mut u32);
        pub fn eth_bufferSet(frame: u32, key_ptr: *const u32, value_ptr: *const u32);
        pub fn eth_bufferMerge(frame_a: u32, frame_b: u32);

        // wSw interface
        pub fn eth2_expose(name: *const u32, length: u32);
        pub fn eth2_loadModule(frame: u32, code: *const u32, length: u32);
        pub fn eth2_callModule(
            frame: u32,
            name: *const u32,
            name_length: u32,
            argument: *const u32,
            argument_length: u32,
            ret: *const u32,
            ret_length: u32,
        );
        pub fn eth2_argument(dest: *const u32, length: u32);
        pub fn eth2_return(src: *const u32, length: u32);
    }
}

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

/// Compute the post-state root given and input `blob` and pre-state root.
///
/// Blob format:
/// +--------------+------------------+------------+
/// | Proof offset | Transaction data | Proof data |
/// +--------------+------------------+------------+
///     4 bytes         T bytes            P bytes
pub fn process_data_blob(blob: &mut [u8], pre_state_root: &[u8; 32]) -> [u8; 32] {
    let (mem_offset, blob) = blob.split_at_mut(4);
    let mem_offset = u32::from_le_bytes(*array_ref!(mem_offset, 0, 4)) as usize;
    let (transactions, blob) = blob.split_at_mut(mem_offset);

    let mut mem = unsafe { Oof::from_blob(blob.as_mut_ptr(), 4) };

    // Verify pre_state_root
    let pre_root = mem.root().unwrap();
    assert_eq!(pre_state_root, pre_root);

    process_raw_transactions(&mut mem, transactions).expect("to process all transactions");

    *mem.root().unwrap()
}
