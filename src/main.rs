#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate blog_os;

use blog_os::exit_qemu;

use core::panic::PanicInfo;




/// Entry point, We need to give the linker something to work with.
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello, vga buffer");
    serial_println!("no panic!");

    unsafe { exit_qemu(); }
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
