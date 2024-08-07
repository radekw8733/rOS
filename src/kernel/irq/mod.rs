pub fn enable_interrupts() {
    #[cfg(feature = "x86_64")]
    crate::arch::x86_64::irq::enable_interrupts()
}