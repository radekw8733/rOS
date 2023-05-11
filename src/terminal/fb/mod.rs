use alloc::vec::{Vec};

use crate::{bootboot::{psf2_t, BOOTBOOT_FB}, peripherals::PERIPHERALS};

use super::{GenericConsole, Color};

pub struct FramebufferConsole {
    font: &'static psf2_t,
    // char position in buffer
    charpos_x: u32,
    charpos_y: u32,
    char_buffer: Vec<Vec<u8>>
}

impl GenericConsole for FramebufferConsole {
    fn print_char(&mut self, c: char) {
        let fb = &PERIPHERALS.read();
        let fb = fb.framebuffer.as_ref().unwrap();
        if c == '\n' {
            self.charpos_x = 0;
            
            if self.charpos_y * self.font.height < fb.fb_height - self.font.height {
                self.charpos_y += 1;
            }
            else {
                self.scroll();
            }
        }
        else {
            if self.charpos_x * self.font.width > fb.fb_width {
                self.charpos_y += 1;
                self.charpos_x = 2;
            }
            self.char_buffer[self.charpos_y as usize][self.charpos_x as usize] = c as u8;
            self.render_char(c, self.charpos_x * self.font.width, self.charpos_y * self.font.height);
            self.charpos_x += 1;
        }
    }
}

impl FramebufferConsole {
    pub fn new(font: &'static psf2_t) -> FramebufferConsole {
        let fb = &PERIPHERALS.read();
        let fb = fb.framebuffer.as_ref().unwrap();
        let width = fb.fb_width / font.width;
        let height = fb.fb_height / font.height;
        // let width = 101;
        // let height = 48;
        let array = vec![vec![b' '; width as usize]; height as usize];
        // let array = vec![vec![' '; 5]; 5];
        FramebufferConsole {
            font,
            charpos_x: 0,
            charpos_y: 0,
            char_buffer: array
        }
    }

    pub fn render_char(&mut self, c: char, pixel_x: u32, pixel_y: u32) {
        self.render_char_u8(c as u8, pixel_x, pixel_y);
    }

    fn render_char_u8(&mut self, c: u8, pixel_x: u32, pixel_y: u32) {
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
    }

    pub fn scroll(&mut self) {
        self.scrolls(1)
    }

    pub fn scrolls(&mut self, n: usize) {
        // clear first n lines
        for y in 0..n {
            self.char_buffer[y].fill(b' ');
        }
        for y in n..self.char_buffer.len() {
            for x in 0..self.char_buffer[y].len() {
                self.char_buffer[y - n][x] = self.char_buffer[y][x];
                self.char_buffer[y][x] = b' ';
            }
        }

        for y in 0..self.char_buffer.len() {
            for x in 0..self.char_buffer[y].len() {
                self.render_char(' ', x as u32 * self.font.width, y as u32 * self.font.height);
                self.render_char_u8(self.char_buffer[y][x], x as u32 * self.font.width, y as u32* self.font.height);
            }
        }
    }
    
    fn _write_pixel(&self, addr: usize, col: Color) {
        let ptr = addr as *mut u32;
        unsafe { *ptr = col as u32 };
    }

    fn _get_glyph(&self, c: &u8) -> *const u8 {
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
        
        let pitch = 4 * fb_scanline;
        fb_addr + (fb_height - (fb_height - 1 - y)) * pitch * x
    }
}