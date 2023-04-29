pub mod fb;

pub trait Console {
    fn print(&mut self, s: &str);
    fn println(&mut self, s: &str);
    fn print_char(&mut self, c: char);
}

pub enum Color {
    Red = 0xFF0000,
    Green = 0xFF00,
    Blue = 0xFF,
    Black = 0x0
}