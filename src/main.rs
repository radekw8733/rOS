#![allow(non_snake_case)]
#![feature(panic_info_message)]
#![feature(ptr_internals)]
#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use alloc::boxed::Box;
use assembly_macros::halt;
use graphics::{framebuffer::Framebuffer, Size};
use lazy_static::lazy_static;
use memory::MEMORY_MANAGER;
use spin::Mutex;
use tty::{fb::FramebufferConsole, Console};
use gdt::load_gdt;

mod assembly_macros;
mod memory;
mod graphics;
mod gdt;
mod tty;
// mod serial;
// mod interrupts;
// mod timer;
// mod keyboard;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

static FB_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new(0);
lazy_static! {
    static ref CONSOLE: Mutex<Box<dyn Console + Send>> = Mutex::new(Box::new(FramebufferConsole::new({
        let framebuffer_l = &FB_REQUEST.get_response().get().unwrap().framebuffers()[0];
    
        Framebuffer::new(
            framebuffer_l.address.as_ptr().unwrap() as *mut u32,
            Size::new(framebuffer_l.width as u32, framebuffer_l.height as u32))
    })));
}

#[no_mangle]
pub extern "C" fn _start() {
    load_gdt();

    MEMORY_MANAGER.lock().add_mem_to_heap(256); // 1MB
    MEMORY_MANAGER.lock().print_heap_usage();

    halt();
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    unsafe { CONSOLE.force_unlock() };
    print!("\n{}", panic_info);

    halt();
    loop {}
}