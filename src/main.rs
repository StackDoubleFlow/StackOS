#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::WRITER.write_string("Hello world!");

    loop {}
}