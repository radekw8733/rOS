use super::{Size, Point, Color};

pub struct Framebuffer {
    address: *mut u32,
    pub width: u64,
    pub height: u64
}

impl Framebuffer {
    pub fn new(address: *mut u32, size: Size) -> Self {
        Self {
            address,
            width: size.width,
            height: size.height
        }
    }

    pub fn write_pixel(&mut self, pixel: Point, color: Color) {
        unsafe {
            let offset = pixel.y as u64 * self.width + pixel.x as u64;
            *(self.address.offset((offset * 4) as isize)) = color.pack();
        }
    }

    pub fn fill(&mut self, color: Color) {
        let iter = self.width * self.height;
        let c = color.pack();

        for i in 0..iter {
            unsafe { *(self.address.offset(i as isize)) = c }
        }
    }
}