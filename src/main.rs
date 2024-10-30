#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod vga;
mod device;
mod interrupt;

use crate::device::cpu::{get_processor_name_str, get_processor_vendor_str};
use crate::interrupt::idt;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Core died: {}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Starting kernel...");

    idt::init_idt();

    unsafe {
        println!("[CPUID] CPU Vendor: {:?}", get_processor_vendor_str());
        println!("[CPUID] CPU Name: {:?}", get_processor_name_str());
    }

    loop {}
}

