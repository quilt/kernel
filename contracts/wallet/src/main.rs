#![no_std]
#![no_main]

extern "C" {
    fn print(str_ptr: *const u32, len: u32);
}

#[no_mangle]
fn main() {
    let msg = "Hello World";

    unsafe {
        print(msg.as_ptr() as *const u32, msg.len() as u32);
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
