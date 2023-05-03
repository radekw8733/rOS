use spin::RwLock;

use crate::{terminal::fb::Framebuffer, bootboot::{BOOTBOOT_INFO, BOOTBOOT}};

pub struct Peripherals {
    pub framebuffer: Option<Framebuffer>,
    // pub serial_port: Option<SerialPort>
}

// initialize on start rather than statically
pub static PERIPHERALS: RwLock<Peripherals> = RwLock::new(Peripherals {
    framebuffer: None,
    // serial_port: None
});

impl Peripherals {
    pub fn init(&mut self) {
        // self.init_serial_port();
        self.init_framebuffer();
    }

    fn init_framebuffer(&mut self) {
        if self.framebuffer.is_none() {
            let bootboot_info = unsafe { &*(BOOTBOOT_INFO as *const BOOTBOOT) };

            self.framebuffer = Some(Framebuffer { 
                fb_addr: bootboot_info.fb_ptr as usize,
                fb_size: bootboot_info.fb_size,
                fb_width: bootboot_info.fb_width,
                fb_height: bootboot_info.fb_height,
                fb_scanline: bootboot_info.fb_scanline
            });
        }
    }

    // fn init_serial_port(&mut self) {
    //     if self.serial_port.is_none() {
    //         self.serial_port = Some(SerialPort::get_first_port());
    //     }
    // }
}