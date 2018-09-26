

#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

extern crate blog_os;

use core::panic::PanicInfo;
use blog_os::exit_qemu;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // test goes ihr
    unsafe { exit_qemu(); }
    loop {}
}
#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
