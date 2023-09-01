#![allow(non_snake_case)]
#![feature(panic_info_message)]
#![feature(ptr_internals)]
#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

// extern crate alloc;

use core::{panic::PanicInfo, arch::{global_asm, asm}};
use graphics::{framebuffer::Framebuffer, Size, Color};
// use terminal::CONSOLE;
use x86::gdt::load_gdt;
use x86_64::{instructions::interrupts::enable, structures::paging::frame};

// use crate::{assembly_macros::halt, terminal::shell::Shell};
// mod terminal;
// mod diagnostics;
// mod assembly_macros;
// mod memory;
// mod serial;
// mod interrupts;
// mod timer;
// mod keyboard;
mod graphics;
mod x86;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new(0);

// boostrap processor entry
#[no_mangle]
pub extern "C" fn _start() {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response().get() {
        let framebuffer_l = &framebuffer_response.framebuffers()[0];

        let mut framebuffer = Framebuffer::new(
            framebuffer_l.address.as_ptr().unwrap() as *mut u32,
            Size::new(framebuffer_l.width, framebuffer_l.height));
    }

    load_gdt();

    // PERIPHERALS.write().init();
    // SUBSYSTEMS.write().init();

    // println!("boot complete...");
    // println!();

    // diagnostics::print_diagnostics();

    // // let's test division error handler
    // println!("testing IDT division handler...");

    // // this loops
    // // unsafe { 
    // //     asm!(
    // //         "mov eax, 0",
    // //         "div eax"
    // //     );
    // // }
    // // this does not
    // unsafe { asm!("int 0"); }

    // enable();
    // Shell::main_loop();
    // halt();

    loop {}
}

// // code to run on application processors
// #[no_mangle]
// pub extern "C" fn ap() {
//     loop {}
// }

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    // unsafe { CONSOLE.force_unlock() };
    // print!("{}", panic_info);
    loop {}
}