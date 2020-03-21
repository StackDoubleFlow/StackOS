#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

extern crate alloc;

mod allocator;
mod gdt;
mod interrupts;
mod memory;
mod vga;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

pub fn hlt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt();
}

fn init() {
    interrupts::init_idt();
    gdt::init();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use alloc::boxed::Box;
    use memory::BootInfoFrameAllocator;
    use x86_64::VirtAddr;

    init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    println!(r"
    Welcome to
     _____  _                _      ____    _____         ___    __ 
    / ____|| |              | |    / __ \  / ____|       / _ \  /_ |
   | (___  | |_  __ _   ___ | | __| |  | || (___ __   __| | | |  | |
    \___ \ | __|/ _` | / __|| |/ /| |  | | \___ \\ \ / /| | | |  | |
    ____) || |_| (_| || (__ |   < | |__| | ____) |\ V / | |_| |_ | |
   |_____/  \__|\__,_| \___||_|\_\ \____/ |_____/  \_/   \___/(_)|_|
    ");
    

    hlt();
}
