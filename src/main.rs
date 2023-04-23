#![allow(non_snake_case)]
#![no_std]
#![no_main]

use core::{panic::PanicInfo, arch::global_asm};

mod terminal;

global_asm!(include_str!("multiboot.S"));

#[no_mangle]
pub extern "C" fn kmain() {
    terminal::vga::print("asd");
}

#[panic_handler]
fn panic(p: &PanicInfo) -> ! {
    terminal::vga::print(p.payload().downcast_ref::<&str>().unwrap());
    loop {}
}