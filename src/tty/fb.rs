use alloc::vec::Vec;
use alloc::vec;
use psf2::Font;

use crate::graphics::{framebuffer::Framebuffer, Color, Point};

use super::Console;

const FONT_DATA: &'static [u8] = include_bytes!("font.psf");

pub struct FramebufferConsole {
    // char position in buffer
    charpos_x: u32,
    charpos_y: u32,
    fb: Framebuffer,
    font: Font<&'static [u8]>,
    char_buffer: Vec<Vec<u8>>
}

impl Console for FramebufferConsole {
    fn write(&mut self, c: char) {
        if c == '\n' {
            self.charpos_x = 0;
            
            if self.charpos_y * self.font.height() < self.fb.height - self.font.height() {
                self.charpos_y += 1;
            }
            else {
                self.scroll();
            }
        }
        else {
            if self.charpos_x * self.font.width() >= self.fb.width {
                self.charpos_y += 1;
                self.charpos_x = 2;
            }
            self.char_buffer[self.charpos_y as usize][self.charpos_x as usize] = c as u8;
            self.render_char(c, self.charpos_x * self.font.width(), self.charpos_y * self.font.height());
            self.charpos_x += 1;
        }
    }
}

impl core::fmt::Write for (dyn Console + Send + 'static) {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}

impl FramebufferConsole {
    pub fn new(fb: Framebuffer) -> FramebufferConsole {
        let font = Font::new(FONT_DATA).unwrap();
        let width = fb.width / font.width();
        let height = fb.height / font.height();
        // let width = 101;
        // let height = 48;
        let array = vec![vec![b' '; width as usize]; height as usize];
        // let array = vec![vec![' '; 5]; 5];
        FramebufferConsole {
            charpos_x: 0,
            charpos_y: 0,
            fb,
            font,
            char_buffer: array
        }
    }

    fn render_char(&mut self, c: char, pixel_x: u32, pixel_y: u32) {
        self.render_char_u8(c as u8, pixel_x, pixel_y);
    }

    fn render_char_u8(&mut self, c: u8, pixel_x: u32, pixel_y: u32) {
        let font_size = (self.font.width(), self.font.height());
        let glyph = self.font.get_ascii(c).unwrap();

        for (y, row) in glyph.enumerate() {
            for glyph_row in row.data().iter() {
                for x in 0..8 {
                    let color = match glyph_row & (1 << x) {
                        0 => Color::new(0, 0, 0),
                        _ => Color::new(0, 255, 0)
                    };
                    let pixel = Point::new((font_size.0 - x as u32) + pixel_x, y as u32 + pixel_y);
                    
                    self.fb.write_pixel(pixel, color)
                }
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
                self.render_char(' ', x as u32 * self.font.width(), y as u32 * self.font.height());
                self.render_char_u8(self.char_buffer[y][x], x as u32 * self.font.width(), y as u32* self.font.height());
            }
        }
    }
}