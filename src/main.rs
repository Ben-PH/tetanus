#![feature(panic_implementation)]
#![no_std]
#![no_main]

extern crate bootloader_precompiled;
extern crate volatile;
extern crate spin;


#[macro_use]
extern crate lazy_static;

#[macro_use]
mod vga_buffer;

use core::panic::PanicInfo;


#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// Entry point, We need to give the linker something to work with.
#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("hello, there");
    println!();
    println!("this has {} extra args", 1);
    panic!("Panic! at the disco");
}
