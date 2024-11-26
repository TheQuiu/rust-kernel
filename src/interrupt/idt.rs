use x86_64::structures::idt::{DivergingHandlerFuncWithErrCode, InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;

use crate::println;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt.overflow.set_handler_fn(stack_overflow_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame
) {
    println!("[ERROR]: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("[ERROR]: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn stack_overflow_handler(
    stack_frame: InterruptStackFrame
) {
    panic!("[ERROR]: STACK OVERFLOW\n{:#?}", stack_frame);
}