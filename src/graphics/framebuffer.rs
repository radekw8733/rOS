use limine::NonNullPtr;

use super::{Size, Point, Color};

pub struct Framebuffer {
    address: usize,
    pub size: Size
}

impl Framebuffer {
    pub fn new_limine(framebuffer_l: &NonNullPtr<limine::Framebuffer>) -> Self {
        let address = framebuffer_l.address.as_ptr().unwrap() as usize;
        let size = Size::new(framebuffer_l.width as u32, framebuffer_l.height as u32);
        Self {
            address,
            size
        }
    }

    pub fn write_pixel(&mut self, pixel: Point, color: Color) {
        unsafe {
            let offset = pixel.y * self.size.width + pixel.x;
            *((self.address as *mut u32).offset(offset as isize)) = color.pack();
        }
    }

    pub fn _fill(&mut self, color: Color) {
        let iter = self.size.width * self.size.height;
        let c = color.pack();

        for i in 0..iter {
            unsafe { *((self.address as *mut u32).offset(i as isize)) = c }
        }
    }
}