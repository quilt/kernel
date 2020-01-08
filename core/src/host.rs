#[cfg(not(test))]
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
    ) -> u32;
    pub fn eth2_argument(dest: *const u32, length: u32);
    pub fn eth2_return(src: *const u32, length: u32);

    // debug
    pub fn print(ptr: *const u32, len: u32);
}
