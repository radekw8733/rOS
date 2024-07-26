use spin::Mutex;
use x86_64::structures::idt::InterruptStackFrame;

use crate::arch::x86_64::irq::pic::{PICInterrupt, PIC};


pub static _PIT: Mutex<PITType> = Mutex::new(PITType);

pub struct PITType;

impl PITType {
    pub extern "x86-interrupt" fn interrupt_handler(_stack: InterruptStackFrame) {
        PIC.lock().eoi(PICInterrupt::PIT);
    }
}