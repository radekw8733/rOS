use x86_64::structures::idt::InterruptStackFrame;

use crate::{interrupts::pic::PIC, assembly_macros::io_in, terminal::shell::Shell};

const PIC_PS2K_IRQ_LINE: u8 = 1;

pub struct PS2ControllerType;

impl PS2ControllerType {
    pub fn new() -> PS2ControllerType {
        PIC.lock().unmask(PIC_PS2K_IRQ_LINE);
        PS2ControllerType
    }

    pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack: InterruptStackFrame) {
        let key = io_in(0x60);
        Shell::pass_key(key);

        PIC.lock().eoi(PIC_PS2K_IRQ_LINE);
    }
}