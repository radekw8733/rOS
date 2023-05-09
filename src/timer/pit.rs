// doesn't exist on newer hardware :(
use x86_64::{structures::idt::InterruptStackFrame, instructions::interrupts};

use crate::{terminal::CONSOLE, interrupts::pic::PIC, print};

const PIC_PIT_IRQ_LINE: u8 = 0;

pub struct PIT;

impl PIT {
    pub fn new() -> PIT {
        PIC.lock().unmask(PIC_PIT_IRQ_LINE);
        PIT
    }

    pub extern "x86-interrupt" fn interrupt_handler(_stack: InterruptStackFrame) {
        unsafe { CONSOLE.force_unlock() };
        interrupts::without_interrupts(|| {
            print!(".");
        });
        PIC.lock().eoi(PIC_PIT_IRQ_LINE);
    }
}