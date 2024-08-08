use crate::{drivers::io::pci::print_pci_devices, kernel::mm::phys_allocator::PHYS_ALLOCATOR};

pub fn run_diagnostics() {
    log_memory();
    // print_pci_devices();

    log::info!("booting rOS kernel complete");
}

fn log_memory() {
    log::info!("available physical memory: {}", humansize::format_size(PHYS_ALLOCATOR.lock().get().unwrap().available_bytes(), humansize::DECIMAL));
    log::info!("allocated physical memory: {}", humansize::format_size(PHYS_ALLOCATOR.lock().get().unwrap().allocated_bytes(), humansize::DECIMAL));
}