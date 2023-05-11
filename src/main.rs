#![allow(non_snake_case)]
#![feature(panic_info_message)]
#![feature(ptr_internals)]
#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

use core::{panic::{PanicInfo}, arch::{global_asm, asm}};
use peripherals::PERIPHERALS;
use subsystems::SUBSYSTEMS;
use terminal::CONSOLE;
use x86_64::instructions::interrupts::enable;

use crate::{assembly_macros::halt, terminal::shell::Shell};
mod bootboot;
mod terminal;
mod peripherals;
mod subsystems;
mod diagnostics;
mod assembly_macros;
mod memory;
mod serial;
mod interrupts;
mod timer;
mod keyboard;

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

    println!("boot complete...");
    println!();

    diagnostics::print_diagnostics();

    // let's test division error handler
    println!("testing IDT division handler...");

    // this loops
    // unsafe { 
    //     asm!(
    //         "mov eax, 0",
    //         "div eax"
    //     );
    // }
    // this does not
    unsafe { asm!("int 0"); }

    enable();
    Shell::main_loop();
    halt();
}

// code to run on application processors
#[no_mangle]
pub extern "C" fn ap() {
    loop {}
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    unsafe { CONSOLE.force_unlock() };
    print!("{}", panic_info);
    loop {}
}