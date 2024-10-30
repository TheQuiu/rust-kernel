#![no_std]
#![no_main]

mod vga;
mod device;

use core::panic::PanicInfo;
use crate::device::cpu::CpuId;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Core died: {}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Starting kernel...");
    let cpu = unsafe { CpuId::new() };

    let cpu_vendor = cpu.print_cpu_vendor();
    println!("[CPUID] CPU Vendor: ");
    cpu_vendor.iter().for_each(|char| print!("{}", char));



    loop {}
}

