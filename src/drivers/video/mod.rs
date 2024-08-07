pub mod framebuffer;

#[derive(Default, Clone, Copy)]
pub struct Size {
    pub width: u32,
    pub height: u32
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self { Self { width, height } }

    pub fn mem_size(&self) -> usize {
        (self.width * self.height) as usize
    }
}

#[derive(Default, Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self { Self { x, y } }

    pub fn is_inside(&self, size: &Size) -> bool {
        self.x < size.width && self.y < size.height
    }
}

#[derive(Default, Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

impl Color {
    pub fn pack(&self) -> u32 {
        self.blue as u32 | ((self.green as u32) << 8) | ((self.red as u32) << 16)
    }
}

pub struct Pixel {
    pub point: Point,
    pub color: Color,
}

pub trait Framebuffer {
    fn size(&self) -> Size;
    fn plot(&mut self, pixel: Pixel);
    fn fill(&mut self, color: Color);
    fn fill_rect(&mut self, origin: Pixel, size: Size) {
        for y in origin.point.y..size.height {
            for x in origin.point.x..size.width {
                self.plot(Pixel {
                    point: Point { x, y },
                    color: origin.color
                });
            }
        }
    }
    fn display(&mut self);
}