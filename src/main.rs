#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

mod vga;
mod interrupts;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn init() {
    interrupts::init_idt();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    println!("Hello World!, here are some numbers: {} {}", 123, 3.14159);
    x86_64::instructions::interrupts::int3();
    println!("works");

    loop {}
}