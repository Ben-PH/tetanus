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
extern crate x86_64;

#[macro_use]
mod vga_buffer;

#[macro_use]
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

    println!("Hello, vga buffer");
    serial_println!("Hello, {}, I am {}", "host", "serial");
    serial_println!("super serial");
    serial_println!();
    unsafe {exit_qemu(); }
    loop {}
}

// unsafe: QEMU defice on IO port addr 0xf4
pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4); // 19
    port.write(0);
}
