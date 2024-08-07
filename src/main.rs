#![allow(non_snake_case)]
#![feature(abi_x86_interrupt)]
#![feature(vec_into_raw_parts)]
#![feature(slice_from_ptr_range)]
#![feature(debug_closure_helpers)]
#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;

use arch::ops::halt;
use kernel::diagnostics::run_diagnostics;

use crate::kernel::{init, log::LOGGER};

mod arch;
mod drivers;
mod kernel;

const _VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // GDT needs work to do with Limine https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#x86_64-1
    // load_gdt();

    init::init();
    run_diagnostics();
    halt();
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    unsafe { LOGGER.force_unlock() };
    log::error!(target: "KERNEL PANIC","{}", panic_info);

    halt();
}