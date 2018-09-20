#![feature(panic_implementation)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate std;

extern crate bootloader_precompiled;
extern crate volatile;
extern crate spin;
extern crate uart_16550;


#[macro_use]
extern crate lazy_static;

#[macro_use]
mod vga_buffer;
mod serial;


use core::panic::PanicInfo;
#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// Entry point, We need to give the linker something to work with.
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("hello, there");
    println!();
    println!("this has {} extra args", 1);
    panic!("Panic! at the disco");
}
