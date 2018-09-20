#![feature(panic_implementation)]
#![no_std]
#![no_main]

extern crate bootloader_precompiled;
extern crate volatile;

#[macro_use]
extern crate lazy_static;

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

/// Entry point, We need to give the linker something to work with.
#[no_mangle]
pub extern "C" fn _start() -> ! {

    vga_buffer::Writer::print_something();
    loop {}
}
