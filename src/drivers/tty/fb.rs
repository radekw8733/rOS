use alloc::vec::Vec;
use alloc::{collections::VecDeque, string::ToString};
use alloc::string::String;
use psf2::Font;

use crate::{drivers::video::{framebuffer::GenericFramebuffer, Color, Framebuffer, Pixel, Point, Size}, kernel::log::Console};

const FONT_DATA: &'static [u8] = include_bytes!("font.psf");

pub struct FramebufferConsole {
    fb: GenericFramebuffer,
    font: Font<&'static [u8]>,
    buf: VecDeque<String>,
    current_foreground: Color,
    current_background: Color,
    is_in_escape_mode: bool,
    current_escape_seq: String,
}

impl Console for FramebufferConsole {
    fn write(&mut self, c: char) {
        self.add_to_buffer(c);
    }

    fn write_str(&mut self, s: &str) {
        for c in s.chars() {
            self.add_to_buffer(c);
        }
    }
}

impl FramebufferConsole {
    pub fn new(fb: GenericFramebuffer) -> FramebufferConsole {
        let font = Font::new(FONT_DATA).unwrap();
        let height = fb.size().height / font.height();

        FramebufferConsole {
            fb,
            font,
            buf: VecDeque::with_capacity(height as usize),
            current_foreground: Color { red: 255, green: 255, blue: 255 },
            current_background: Color { red: 0, green: 0, blue: 0 },
            is_in_escape_mode: false,
            current_escape_seq: String::new(),
        }
    }

    fn add_to_buffer(&mut self, c: char) {
        match self.buf.len() {
            0 => self.buf.push_back(String::new()),
            len if len as u32 > self.fb.size().height / self.font.height() => { 
                drop(self.buf.pop_front());
                self.rerender();
            },
            _ => ()
        }
        if c == '\n' {
            self.buf.push_back(String::new());
        }
        else {
            self.buf.back_mut().unwrap().push(c);
            self.rerender_line(self.buf.len() as u32 - 1);
        }
    }

    fn rerender(&mut self) {
        self.fb.fill(Color { red: 0, green: 0, blue: 0 });

        let buf = core::mem::replace(&mut self.buf, VecDeque::new());
        for (y, line) in buf.iter().enumerate() {
            let mut x = 0;
            for c in line.chars() {
                if self.parse_char(
                    c,
                    Point {
                        x: x as u32 * self.font.width(),
                        y: y as u32 * self.font.height(),
                    }
                ) {
                    x += 1;
                }
            }
        }
        self.buf = buf;

        self.fb.display();
    }

    fn rerender_line(&mut self, line: u32) {
        self.fb.fill_rect(
            Pixel {
                color: Color { red: 0, green: 0, blue: 0 },
                point: Point { x: 0, y: line * self.font.height() }
            },
            Size {
                height: self.font.height(),
                width: self.fb.size().width
            }
        );

        let buf = core::mem::replace(&mut self.buf, VecDeque::new());
        let mut x = 0;
        for c in buf[line as usize].chars() {
            if self.parse_char(
                c,
                Point {
                    x: x as u32 * self.font.width(),
                    y: line as u32 * self.font.height(),
                }
            ) {
                x += 1;
            }
        }
        self.buf = buf;

        self.fb.display();
    }

    fn parse_char(&mut self, c: char, point: Point) -> bool {
        match c {
            '\u{001b}' => {
                self.is_in_escape_mode = true;
                self.current_escape_seq = c.to_string();
                false
            },
            'm' => {
                match self.is_in_escape_mode {
                    true => {
                        self.is_in_escape_mode = false;
                        let mut params = self.current_escape_seq[2..].split(';');
                        // TODO: needs safe value check
                        match params.nth(0).unwrap() {
                            "38" => {
                                let colors: Vec<&str> = params.skip(1).collect();
                                self.current_foreground = Color {
                                    red: colors[0].parse().unwrap(),
                                    green: colors[1].parse().unwrap(),
                                    blue: colors[2].parse().unwrap()
                                }
                            },
                            "48" => {
                                let colors: Vec<&str> = params.skip(2).collect();
                                self.current_background = Color {
                                    red: colors[0].parse().unwrap(),
                                    green: colors[1].parse().unwrap(),
                                    blue: colors[2].parse().unwrap()
                                }
                            },
                            _ => {}
                        };
                        false
                    },
                    false => { self.render_char(c, point); true }
                }
            }
            _ => {
                match self.is_in_escape_mode {
                    true => { self.current_escape_seq.push(c); false },
                    false => { self.render_char(c, point); true }
                }
            }
        }
    }

    fn render_char(&mut self, c: char, point: Point) {
        let font_size = (self.font.width(), self.font.height());
        let glyph = self.font.get_ascii(c as u8).unwrap();

        for (y, row) in glyph.enumerate() {
            for glyph_row in row.data().iter() {
                for x in 0..8 {
                    let color = match glyph_row & (1 << x) {
                        0 => self.current_background,
                        _ => self.current_foreground
                    };
                    let point = Point::new((font_size.0 - x as u32) + point.x, y as u32 + point.y);
                    let pixel = Pixel { color, point };
                    
                    self.fb.plot(pixel);
                }
            }
        }
    }
}