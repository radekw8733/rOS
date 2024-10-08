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

use crate::kernel::init::init;

mod arch;
mod drivers;
mod kernel;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    run_diagnostics();

    halt();
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    log::error!(target: "KERNEL PANIC","{}", panic_info);

    halt();
}