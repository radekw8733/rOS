use self::{idt::load_idt, pic::{PIC, PICInterrupt}};

#[cfg(target_arch = "x86_64")]
pub mod pic;
#[cfg(target_arch = "x86_64")]
pub mod idt;
#[cfg(target_arch = "x86_64")]
pub mod exceptions;

pub fn load_interrupts() {
    load_idt();

    PIC.lock().init();
    PIC.lock().unmask(PICInterrupt::PIT);

    x86_64::instructions::interrupts::enable();
}