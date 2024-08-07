use crate::{kernel::irq::enable_interrupts, print, println};

use super::log::LOGGER;

#[cfg(feature = "limine")]
pub mod limine;

pub fn init() {
    println!("booting rOS...");
    setup_memory_manager();
    println!("memory setup complete");
    switch_logger();
    enable_interrupts();
    log::info!("interrupts enabled");
}

pub fn setup_memory_manager() {
    #[cfg(all(target_arch = "x86_64", feature = "limine"))]
    {
        limine::setup_memory();
    }
}

pub fn switch_logger() {
    println!("enabling framebuffer");
    #[cfg(feature = "limine")]
    limine::add_framebuffer_console();

    print!("switching to dynamic logger");
    LOGGER.lock().switch_to_allocated_mode();
}