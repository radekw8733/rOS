use spin::RwLock;
use x86_64::{set_general_handler, structures::idt::{InterruptDescriptorTable, InterruptStackFrame}};

pub static IDT: RwLock<InterruptDescriptorTable> = RwLock::new(InterruptDescriptorTable::new());

pub fn exception_handler(stack: InterruptStackFrame, index: u8, _error_code: Option<u64>) {
    if index == 8 {
        panic!("-- DOUBLE FAULT EXCEPTION --\n{:#?}", stack);
    }
    log::error!("general exception nr: {}", index);
}

pub fn load_idt() {
    let mut idt = InterruptDescriptorTable::new();
    // handle only double faults for now
    set_general_handler!(&mut idt, exception_handler, 8);

    unsafe {
        *IDT.write() = idt;
        IDT.read().load_unsafe();
    }
}