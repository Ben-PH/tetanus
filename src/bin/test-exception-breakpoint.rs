#![feature(abi_x86_interrupt)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate blog_os;
extern crate x86_64;
#[macro_use]
extern crate lazy_static;

use blog_os::exit_qemu;
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicUsize, Ordering};

static BREAKPOINT_HAND_CALLED: AtomicUsize = AtomicUsize::new(0);

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start () -> ! {

    init_idt();

    x86_64::instructions::int3();

    match BREAKPOINT_HAND_CALLED.load(Ordering::SeqCst) {
        1 => serial_println!("ok"),
        0 => {
            serial_println!("Fail");
            serial_println!("Breakpoint handler not called");
        }
        other => {
            serial_println!("Fail");
            serial_println!("Breakpoint handler called {} times", other);
        }
    }

    unsafe {exit_qemu();}

    loop{}
}

#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    serial_println!("failed");
    serial_println!("{}", info);

    unsafe {
        exit_qemu();
    }

    loop {}
}

use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(_: &mut ExceptionStackFrame) {
    BREAKPOINT_HAND_CALLED.fetch_add(1, Ordering::SeqCst);
}
