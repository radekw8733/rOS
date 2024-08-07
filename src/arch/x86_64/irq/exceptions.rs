use x86_64::structures::idt::InterruptStackFrame;

use crate::eprintln;

pub extern "x86-interrupt" fn double_fault_handler(stack: InterruptStackFrame, _zero: u64) -> ! {
    eprintln!("\n-- DOUBLE FAULT EXCEPTION --");
    print_error(stack);
    panic!("double fault exception");
}

pub extern "x86-interrupt" fn division_error_handler(stack: InterruptStackFrame) {
    eprintln!("\n-- DIVISION BY ZERO EXCEPTION --");
    print_error(stack);
}

fn print_error(stack: InterruptStackFrame) {
    eprintln!("{:#?}", stack)
}