#![cfg_attr(not(test), no_std)]

use arrayref::array_ref;
use oof::Oof;

// #[cfg(feature = "ewasm")]
mod native {
    extern "C" {
        pub fn eth2_loadPreStateRoot(offset: *const u32);
        pub fn eth2_blockDataSize() -> u32;
        pub fn eth2_blockDataCopy(outputOfset: *const u32, offset: u32, length: u32);
        pub fn eth2_savePostStateRoot(offset: *const u32);
    }
}

// #[cfg(feature = "ewasm")]
#[no_mangle]
pub extern "C" fn main() {
    let input_size = unsafe { native::eth2_blockDataSize() as usize };

    let mut input = [0u8; 42000];
    unsafe {
        native::eth2_blockDataCopy(input.as_mut_ptr() as *mut u32, 0, input_size as u32);
    }

    let mut pre_state_root = [0u8; 32];
    unsafe { native::eth2_loadPreStateRoot(pre_state_root.as_mut_ptr() as *const u32) }

    let post_root = process_data_blob(&mut input, &pre_state_root);

    unsafe { native::eth2_savePostStateRoot(post_root.as_ptr() as *const u32) }
}

// Blob format:
// +--------------+------------------+------------+
// | Proof offset | Transaction data | Proof data |
// +--------------+------------------+------------+
//     4 bytes         T bytes            P bytes
pub fn process_data_blob(blob: &mut [u8], pre_state_root: &[u8; 32]) -> [u8; 32] {
    let mem_offset = u32::from_le_bytes(*array_ref!(blob, 0, 4)) as usize;
    let transactions = &blob[0..mem_offset];

    let mut mem = unsafe { Oof::from_blob(blob.as_mut_ptr().offset(mem_offset as isize), 4) };

    // Verify pre_state_root
    let pre_root = mem.root().unwrap();
    assert_eq!(pre_state_root, pre_root);

    *mem.root().unwrap()
}
