use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::{keyboard::ps2::PS2ControllerType, timer::pit::PIT, interrupts::exceptions::{double_fault_handler, division_error_handler}};

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt.divide_error.set_handler_fn(division_error_handler);
        idt[33].set_handler_fn(PS2ControllerType::keyboard_interrupt_handler);
        idt[32].set_handler_fn(PIT::interrupt_handler);
        idt
    };
}