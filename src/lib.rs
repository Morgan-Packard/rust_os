#![no_std]
#![feature(abi_x86_interrupt)]

pub mod printing;
pub mod interrupts;

use core::panic::PanicInfo;

pub fn init_os() {
    interrupts::load_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}