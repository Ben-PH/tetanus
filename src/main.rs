#![feature(panic_implementation)]
#![no_std]
#![no_main]


use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Entry point, We need to give the linker something to work with.
    loop {}
}


#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

