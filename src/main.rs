#![allow(non_snake_case)]
#![feature(panic_info_message)]
#![feature(ptr_internals)]
#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;

use assembly_macros::halt;
use interrupts::load_interrupts;
use memory::MEMORY_MANAGER;

use crate::tty::fb::CONSOLE;

mod assembly_macros;
mod memory;
mod graphics;
mod gdt;
mod tty;
// mod serial;
mod interrupts;
mod timer;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // GDT needs work to do with Limine https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#x86_64-1
    // load_gdt();

    MEMORY_MANAGER.lock().add_mem_to_heap(256); // 1MB
    MEMORY_MANAGER.lock().print_heap_usage();

    load_interrupts();
    
    loop { halt() }
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    unsafe { CONSOLE.force_unlock() };
    print!("\n{}", panic_info);

    loop { halt() }
}