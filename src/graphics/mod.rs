pub mod framebuffer;

pub struct Size {
    pub width: u64,
    pub height: u64
}

impl Size {
    pub fn new(width: u64, height: u64) -> Self { Self { width, height } }
}

pub struct Point {
    pub x: u32,
    pub y: u32
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self { Self { x, y } }
}

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self { Self { red, green, blue } }

    pub fn pack(&self) -> u32 {
        self.blue as u32 | ((self.green as u32) << 8) | ((self.red as u32) << 16)
    }
}