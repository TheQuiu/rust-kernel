#![no_std]
#![no_main]

mod vga;

use crate::vga::writer::WRITER;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_hello();
    loop {}
}

fn print_hello() {
    use core::fmt::Write;
    WRITER.lock().write_str("Hello again").unwrap();
    write!(WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}