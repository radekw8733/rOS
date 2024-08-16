pub fn load_interrupts() {
    #[cfg(feature = "x86_64")]
    crate::arch::x86_64::irq::load_interrupts();

    log::info!("interrupts enabled");
}