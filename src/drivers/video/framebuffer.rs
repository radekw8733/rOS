use super::{Color, Framebuffer, Pixel, Point, Size};

pub struct GenericFramebuffer {
    address: usize,
    size: Size,
}

impl GenericFramebuffer {
    pub fn new(address: usize, size: Size) -> Self {
        Self { address, size }
    }

    pub unsafe fn plot_unchecked(&mut self, pixel: Pixel) {
        let offset = pixel.point.y * self.size.width + pixel.point.x;
        *((self.address as *mut u32).offset(offset as isize)) = pixel.color.pack();
    }
}

impl Framebuffer for GenericFramebuffer {
    fn size(&self) -> Size {
        self.size
    }

    fn plot(&mut self, pixel: super::Pixel) {
        if pixel.point.is_inside(&self.size) {
            unsafe { self.plot_unchecked(pixel) }
        }
    }

    fn fill(&mut self, color: Color) {
        for y in 0..self.size.height {
            for x in 0..self.size.width {
                let pixel = Pixel {
                    point: Point { x, y },
                    color,
                };
                unsafe { self.plot_unchecked(pixel) }
            }
        }
    }

    fn display(&mut self) {}
}

// pub struct DoubleBufferedFramebuffer {
//     backbuffer: GenericFramebuffer,
//     framebuffer: GenericFramebuffer,
// }

// impl DoubleBufferedFramebuffer {
//     pub fn new_existing(address: usize, size: Size) -> Self {
//         let allocated_backbuffer = Vec::<u32>::with_capacity(size.mem_size()).into_raw_parts();
//         let backbuffer = GenericFramebuffer::new(allocated_backbuffer.0 as usize, size);
//         let framebuffer = GenericFramebuffer::new(address, size);

//         Self {
//             backbuffer,
//             framebuffer,
//         }
//     }
// }

// impl Framebuffer for DoubleBufferedFramebuffer {
//     fn size(&self) -> Size {
//         self.framebuffer.size()
//     }

//     fn plot(&mut self, pixel: Pixel) {
//         self.backbuffer.plot(pixel);
//     }

//     fn fill(&mut self, color: Color) {
//         self.backbuffer.fill(color);
//     }

//     fn display(&mut self) {
//         unsafe {
//             let backbuffer_ptr = self.backbuffer.address as *mut u32;
//             let backbuffer_slice = slice::from_mut_ptr_range(
//                 backbuffer_ptr..backbuffer_ptr.offset(self.backbuffer.size.mem_size() as isize),
//             );
//             let framebuffer_ptr = self.framebuffer.address as *mut u32;
//             let framebuffer_slice = slice::from_mut_ptr_range(
//                 framebuffer_ptr..framebuffer_ptr.offset(self.backbuffer.size.mem_size() as isize),
//             );
//             backbuffer_slice.swap_with_slice(framebuffer_slice);
//         }
//     }
// }
