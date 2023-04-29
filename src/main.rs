#![allow(non_snake_case)]
#![feature(ptr_internals)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use peripherals::PERIPHERALS;
use subsystems::SUBSYSTEMS;
use terminal::Console;
mod bootboot;
mod terminal;
mod peripherals;
mod subsystems;

#[cfg(target_arch = "aarch64")]
global_asm!(include_str!("../platform/aarch64/rpi4/boot.s"));
#[cfg(target_arch = "aarch64")]
use core::arch::global_asm;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[no_mangle]
pub extern "C" fn kmain() {
    PERIPHERALS.write().init();
    SUBSYSTEMS.write().init();
    SUBSYSTEMS.write().console.as_mut().unwrap().println("initializing systems");

    SUBSYSTEMS.write().console.as_mut().unwrap().println("boot complete...\n");

    SUBSYSTEMS.write().console.as_mut().unwrap().print("Welcome to rOS v: ");
    SUBSYSTEMS.write().console.as_mut().unwrap().print(VERSION);
    SUBSYSTEMS.write().console.as_mut().unwrap().println("!");

    loop {}
}

#[panic_handler]
fn panic(_p: &PanicInfo) -> ! {
    // CONSOLE.print(p.payload().downcast_ref::<&str>().unwrap());
    loop {}
}