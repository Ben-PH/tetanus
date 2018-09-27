#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![feature(abi_x86_interrupt)]

#[macro_use]
extern crate blog_os;
extern crate x86_64;

use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};
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

#[macro_use]
extern crate lazy_static;
lazy_static! {
    pub static ref IDT: InterruptDescriptorTable =  {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_fm: &mut ExceptionStackFrame) {
    println!("EXCePTION: BREAKPOINT\n{:?}", stack_fm);
}
