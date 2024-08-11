#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};

use core::{panic::PanicInfo, fmt::Display};
use rust_os::{printing::*, println};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    rust_os::init_os(); 

    println!("Hello, World!");

    rust_os::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    //println!("{}", info);
    rust_os::hlt_loop();
}