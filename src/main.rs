#![allow(non_snake_case)]
#![feature(panic_info_message)]
#![feature(ptr_internals)]
#![no_std]
#![no_main]

use core::{panic::{PanicInfo}, arch::global_asm};
use peripherals::PERIPHERALS;
use subsystems::SUBSYSTEMS;
use terminal::Console;
mod bootboot;
mod terminal;
mod peripherals;
mod subsystems;
mod diagnostics;
mod assembly_macros;
mod paging;

#[cfg(target_arch = "aarch64")]
global_asm!(include_str!("../platform/aarch64/rpi4/boot.s"));
#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("../platform/x86_64/bootboot/bootstrap.s"));

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// boostrap processor entry
#[no_mangle]
pub extern "C" fn kmain() {
    PERIPHERALS.write().init();
    SUBSYSTEMS.write().init();

    SUBSYSTEMS.write().console.as_mut().unwrap().println("boot complete...\n");

    diagnostics::print_diagnostics();

    SUBSYSTEMS.write().console.as_mut().unwrap().print("\nWelcome to rOS v");
    SUBSYSTEMS.write().console.as_mut().unwrap().print(VERSION);
    SUBSYSTEMS.write().console.as_mut().unwrap().println("!");

    loop {}
}

// code to run on application processors
#[no_mangle]
pub extern "C" fn ap() {
    loop {}
}

#[panic_handler]
fn panic(p: &PanicInfo) -> ! {
    let panic_mess = p.message().unwrap().as_str().unwrap();
    SUBSYSTEMS.write().console.as_mut().unwrap().print(panic_mess);
    loop {}
}