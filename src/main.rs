#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

use core::panic::PanicInfo;


/// Entry point, We need to give the linker something to work with.
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello, vga buffer");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
