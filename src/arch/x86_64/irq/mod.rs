use self::idt::load_idt;

pub mod idt;

pub fn load_interrupts() {
    load_idt();
}