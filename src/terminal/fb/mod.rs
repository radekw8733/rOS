use crate::{bootboot::{psf2_t, BOOTBOOT_FB}, peripherals::PERIPHERALS};

use super::{Console, Color};

pub struct Framebuffer {
    pub fb_addr: usize,
    pub fb_size: u32,
    pub fb_width: u32,
    pub fb_height: u32,
    pub fb_scanline: u32
}

pub struct FramebufferConsole {
    font: &'static psf2_t,
    cursor_x: u32,
    cursor_y: u32,
    spacing: u32
}

impl Console for FramebufferConsole {
    fn print_char(&mut self, c: char) {
        if c == '\n' {
            self.cursor_x = 0;
            self.cursor_y += self.font.height + self.spacing;
        }
        else {
            let font_size = (self.font.width, self.font.height);

            for y in 0..font_size.1 {
                let glyph = unsafe { *self._get_glyph(&c).offset(y as isize) };

                for x in 0..font_size.0 {
                    let color = match glyph & (1 << x) {
                        0 => Color::Black,
                        _ => Color::Green
                    };
                    let pixel = self._get_pixel( (font_size.0 - x) + self.cursor_x, y + self.cursor_y);
                    
                    self._write_pixel(pixel, color);
                }
            }

            self.cursor_x += self.font.width + self.spacing;
        }
    }
}

impl FramebufferConsole {
    pub fn new(font: &'static psf2_t) -> FramebufferConsole {
        FramebufferConsole { font, cursor_x: 0, cursor_y: 0, spacing: 0 }
    }
    
    fn _write_pixel(&self, addr: usize, col: Color) {
        let ptr = addr as *mut u32;
        unsafe { *ptr = col as u32 };
    }

    fn _get_glyph(&self, c: &char) -> *const u8 {
        let font_addr = self.font as *const psf2_t as u64;
        let offset = (((*c as u32) + 2) * (*self.font).bytesperglyph) as u64;
        (font_addr + offset) as *const u8
    }

    fn _get_pixel(&mut self, x: u32, y: u32) -> usize {
        let fb = &PERIPHERALS.read().framebuffer;
        let fb = fb.as_ref().unwrap();
        let fb_addr = BOOTBOOT_FB as usize;

        let x = x as usize;
        let y = y as usize;
        let fb_height = fb.fb_height as usize;
        let fb_scanline = fb.fb_scanline as usize;
        
        fb_addr + (fb_height - (fb_height - 1 - y)) * fb_scanline + 4 * x
    }
}