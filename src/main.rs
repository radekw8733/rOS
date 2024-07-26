#![allow(non_snake_case)]
#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

extern crate alloc;

use core::{panic::PanicInfo, iter::empty};

use alloc::vec::Vec;
use arch::halt;
use interrupts::load_interrupts;
use limine::MemoryMapEntryType;
use memory::{MEMORY_MANAGER, MEMORYMAP_REQUEST};

use crate::tty::fb::CONSOLE;

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

    MEMORY_MANAGER.lock().init();
    MEMORY_MANAGER.lock().print_heap_usage();

    let empty_vec: Vec<u8> = Vec::with_capacity(2);
    MEMORY_MANAGER.lock().print_heap_usage();

    load_interrupts();

    // for device_i in 0..255 {
    //     for function_i in 0..8 {
    //         let pci_device = PCIDeviceSelector::new(0, device_i, function_i);
    //         if is_pci_device_present(&pci_device) {
    //             let pci_dev_info = read_pci_device_id(&pci_device);
    //             println!("{:?}", pci_dev_info);
    //         }
    //     }
    // }

    let mmap = MEMORYMAP_REQUEST.get_response().get().unwrap();
    let mmap = unsafe { core::slice::from_raw_parts(mmap.entries.as_ptr(), mmap.entry_count as usize) };
    let usable_frames = mmap.iter().filter(|r| {
        r.typ == MemoryMapEntryType::Usable || r.typ == MemoryMapEntryType::KernelAndModules
    });
    for frame in usable_frames {
        println!("{:?}", frame);
    }
    
    loop { halt() }
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    unsafe { CONSOLE.force_unlock() };
    print!("\n{}", panic_info);

    loop { halt() }
}