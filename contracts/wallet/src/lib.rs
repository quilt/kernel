pub fn asm() -> Vec<u8> {
    let ret = include_bytes!(concat!(env!("OUT_DIR"), "/main.wasm"));
    ret.to_vec()
}
