#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga::WRITER.lock().write_str("Hello World!").unwrap();
    write!(vga::WRITER.lock(), ", here are some numbers: {} {}", 123, 3.14159).unwrap();

    loop {}
}