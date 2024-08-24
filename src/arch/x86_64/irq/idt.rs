use spin::RwLock;
use x86_64::{set_general_handler, structures::idt::{InterruptDescriptorTable, InterruptStackFrame}};

pub static IDT: RwLock<InterruptDescriptorTable> = RwLock::new(InterruptDescriptorTable::new());

pub fn exception_handler(stack: InterruptStackFrame, index: u8, _error_code: Option<u64>) {
    match index {
        8 => panic!("-- DOUBLE FAULT EXCEPTION --\n{:#?}", stack),
        13 => log::info!("general protection fault at {:x}", stack.instruction_pointer.as_u64()),
        14 => log::info!("page fault at {:x}", stack.instruction_pointer.as_u64()),
        _ => log::error!("unknown exception nr: {}", index)
    }
}

pub fn load_idt() {
    let mut idt = InterruptDescriptorTable::new();
    set_general_handler!(&mut idt, exception_handler);

    unsafe {
        *IDT.write() = idt;
        IDT.read().load_unsafe();
    }
}