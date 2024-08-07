use self::{idt::load_idt, pic::{PICInterrupt, PIC}};

pub mod exceptions;
pub mod idt;
pub mod pic;

pub fn enable_interrupts() {
    load_idt();

    PIC.lock().init();
    PIC.lock().unmask(PICInterrupt::PIT);

    x86_64::instructions::interrupts::enable();
}