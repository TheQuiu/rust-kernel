#![no_std]
#![no_main]

mod vga;
mod device;

use core::arch::asm;
use core::panic::PanicInfo;
use crate::device::cpu::{get_processor_name, print_cpu_vendor};
use crate::vga::writer::DEFAULT_WRITER;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Core died: {}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Starting kernel...");
    unsafe {
        let cpu_vendor = print_cpu_vendor();
        println!("[CPUID] CPU Vendor: ");
        cpu_vendor.iter().for_each(|char| print!("{}", char));

        println!("\n[CPUID] CPU Name: ");
        get_processor_name().iter().for_each(|byte| DEFAULT_WRITER.lock().write_byte(*byte));
    }

    loop {}
}

