use crate::{
    arch::ops::{detect_cpu_speed, prepare_userspace},
    kernel::irq::load_interrupts,
    print, println,
};

use super::log::LOGGER;

#[cfg(feature = "limine")]
pub mod limine;

pub fn init() {
    println!("booting rOS...");

    setup_memory_manager();
    println!("memory setup complete");

    switch_logger();
    load_interrupts();
    detect_cpu_speed();
    prepare_userspace();
}

fn setup_memory_manager() {
    #[cfg(all(target_arch = "x86_64", feature = "limine"))]
    {
        limine::setup_memory();
    }
}

fn switch_logger() {
    println!("enabling framebuffer");
    #[cfg(feature = "limine")]
    limine::add_framebuffer_console();

    print!("switching to dynamic logger");
    LOGGER.lock().switch_to_allocated_mode();
}
