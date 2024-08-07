use crate::{drivers::io::pci::print_pci_devices, kernel::mm::manager::MEMORY_MANAGER};

pub fn run_diagnostics() {
    print_memory();
    MEMORY_MANAGER.lock().get().unwrap().print_mmap_entries();
    // print_pci_devices();

    log::info!("booting rOS kernel complete");
}

fn print_memory() {
    log::info!("available physical memory: {}", humansize::format_size(MEMORY_MANAGER.lock().get().unwrap().phys_memory_available(), humansize::DECIMAL));
    log::info!("allocated physical memory: {}", humansize::format_size(MEMORY_MANAGER.lock().get().unwrap().phys_memory_allocated(), humansize::DECIMAL));
}