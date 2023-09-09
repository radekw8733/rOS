use x86_64::structures::idt::InterruptStackFrame;

use crate::println;

pub extern "x86-interrupt" fn double_fault_handler(stack: InterruptStackFrame, _zero: u64) -> ! {
    println!("\n-- DOUBLE FAULT EXCEPTION --");
    print_error(stack);
    panic!("double fault exception");
}

pub extern "x86-interrupt" fn division_error_handler(stack: InterruptStackFrame) {
    println!("\n-- DIVISION BY ZERO EXCEPTION --");
    print_error(stack);
}

fn print_error(stack: InterruptStackFrame) {
    println!("{:#?}", stack)
}