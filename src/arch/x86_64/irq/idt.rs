use spin::RwLock;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::arch::x86_64::timer::pit::PITType;

use super::{exceptions::{division_error_handler, double_fault_handler}, pic::PICInterrupt};

pub static mut IDT: RwLock<InterruptDescriptorTable> = RwLock::new(InterruptDescriptorTable::new());

pub fn load_idt() {
    let mut idt = InterruptDescriptorTable::new();
    idt[PICInterrupt::PIT.to_idt_entry_index() as u8].set_handler_fn(PITType::interrupt_handler);
    idt.double_fault.set_handler_fn(double_fault_handler);
    idt.divide_error.set_handler_fn(division_error_handler);

    unsafe {
        *IDT.write() = idt;
        IDT.read().load_unsafe();
    }
}