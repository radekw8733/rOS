#![allow(non_snake_case)]
#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;

use arch::halt;
use interrupts::load_interrupts;
use io::pci::{read_pci_config_header, PCIDeviceSelector, read_pci_device_id};
use memory::MEMORY_MANAGER;

use crate::{tty::fb::CONSOLE, io::pci::is_pci_device_present};

mod arch;
mod memory;
mod graphics;
mod gdt;
mod tty;
mod io;
// mod serial;
mod interrupts;
mod timer;

const _VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // GDT needs work to do with Limine https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#x86_64-1
    // load_gdt();

    MEMORY_MANAGER.lock().add_mem_to_heap(256); // 1MB
    MEMORY_MANAGER.lock().print_heap_usage();

    load_interrupts();



    for i in 0..255 {
        let pci_device = PCIDeviceSelector::new(0, i, 0);
        if is_pci_device_present(&pci_device) {
            let pci_dev_info = read_pci_device_id(&pci_device);
            println!("{:?}", pci_dev_info);
        }
    }
    
    loop { halt() }
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    unsafe { CONSOLE.force_unlock() };
    print!("\n{}", panic_info);

    loop { halt() }
}