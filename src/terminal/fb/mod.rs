use crate::{bootboot::{psf2_t, BOOTBOOT_FB}, peripherals::PERIPHERALS};

use super::{GenericConsole, Color};

pub struct Framebuffer {
    pub fb_addr: usize,
    pub fb_size: u32,
    pub fb_width: u32,
    pub fb_height: u32,
    pub fb_scanline: u32
}

pub struct FramebufferConsole {
    font: &'static psf2_t,
    // raw pixel positions
    cursor_x: u32,
    cursor_y: u32,
    spacing: u32
}


impl GenericConsole for FramebufferConsole {
    fn print_char(&mut self, c: char) {
        let fb = &PERIPHERALS.read();
        let fb = fb.framebuffer.as_ref().unwrap();
        if c == '\n' {
            self.cursor_x = 0;
            
            if self.cursor_y < fb.fb_height - self.font.height {
                self.newline();
            }
            else {
                self.scroll();
            }
        }
        else {
            if self.cursor_x > fb.fb_width - self.font.width {
                self.newline();
                self.cursor_x = 2 * (self.font.width + self.spacing);
            }
            self.set_char_at(c, self.cursor_x, self.cursor_y);
        }
    }
}

impl FramebufferConsole {
    pub fn new(font: &'static psf2_t) -> FramebufferConsole {
        FramebufferConsole { font, cursor_x: 0, cursor_y: 0, spacing: 0 }
    }

    pub fn set_char_at(&mut self, c: char, pixel_x: u32, pixel_y: u32) {
        let font_size = (self.font.width, self.font.height);

        for y in 0..font_size.1 {
            let glyph = unsafe { *self._get_glyph(&c).offset(y as isize) };

            for x in 0..font_size.0 {
                let color = match glyph & (1 << x) {
                    0 => Color::Black,
                    _ => Color::Green
                };
                let pixel = self._get_pixel( (font_size.0 - x) + pixel_x, y + pixel_y);
                
                self._write_pixel(pixel, color);
            }
        }

        self.cursor_x += self.font.width + self.spacing;
    }

    pub fn scroll(&mut self) {
        self.scrolls(1)
    }

    fn newline(&mut self) {
        self.cursor_y += self.font.height + self.spacing
    }

    pub fn scrolls(&mut self, count: u32) {
        let fb = &PERIPHERALS.read();
        let fb = fb.framebuffer.as_ref().unwrap();

        for y in 0..count {
            let y = y * (self.font.height + self.spacing);
            for x in 0..fb.fb_width / (self.font.width + self.spacing) {
                let x = x * (self.font.width + self.spacing);
                self.set_char_at(' ', x, y);
            }
        }

        let count  = count * self.font.height;
        for index_y in count..fb.fb_height {
            for index_x in 0..fb.fb_width / 2 {
                unsafe {
                    let spix = self._get_pixel(index_x * 2, index_y - count) as *mut u64;
                    let dpix = self._get_pixel(index_x * 2, index_y) as *mut u64;
                    *spix = *dpix;
                }
            }
        }

        self.cursor_y -= count;
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