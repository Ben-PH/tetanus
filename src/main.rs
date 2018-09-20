#![feature(panic_implementation)]
#![no_std]
#![no_main]

extern crate bootloader_precompiled;
extern crate volatile;
extern crate spin;


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

    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello, there").unwrap();
    write!(vga_buffer::WRITER.lock(), "I've given you {} and {}... Mwahahaha!", 42, 1.0/10.0).unwrap();
    write!(vga_buffer::WRITER.lock(), "and now a new line!!!!!!!!!!!!!!!!!!\nanother!").unwrap();
    loop {}
}
