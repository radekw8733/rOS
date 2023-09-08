use super::{Size, Point, Color};

pub struct Framebuffer {
    address: usize,
    pub width: u32,
    pub height: u32
}

impl Framebuffer {
    pub fn new(address: *mut u32, size: Size) -> Self {
        Self {
            address: address as usize,
            width: size.width,
            height: size.height
        }
    }

    pub fn write_pixel(&mut self, pixel: Point, color: Color) {
        unsafe {
            let offset = pixel.y * self.width + pixel.x;
            *((self.address as *mut u32).offset(offset as isize)) = color.pack();
        }
    }

    // pub fn fill(&mut self, color: Color) {
    //     let iter = self.width * self.height;
    //     let c = color.pack();

    //     for i in 0..iter {
    //         unsafe { *((self.address as *mut u32).offset(i as isize)) = c }
    //     }
    // }
}