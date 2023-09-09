use spin::RwLock;
use x86_64::structures::idt::InterruptDescriptorTable;
use crate::timer::PITType;
use super::pic::PICInterrupt;

pub static mut IDT: RwLock<InterruptDescriptorTable> = RwLock::new(InterruptDescriptorTable::new());

pub fn load_idt() {
    let mut idt = InterruptDescriptorTable::new();
    idt[PICInterrupt::PIT.to_idt_entry_index()].set_handler_fn(PITType::interrupt_handler);

    unsafe {
        *IDT.write() = idt;
        IDT.read().load_unsafe();
    }
}