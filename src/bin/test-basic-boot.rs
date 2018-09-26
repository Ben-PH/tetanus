#![feature(panic_handler)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate blog_os;

use core::panic::PanicInfo;
use blog_os::exit_qemu;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("started, next line is exit_qemu");
    unsafe {exit_qemu();}
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    serial_println!("failed");

    serial_println!("{}", info);

    unsafe { exit_qemu(); }
    loop {}
}
