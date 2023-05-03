#![allow(dead_code)]

use numtoa::NumToA;
pub mod fb;
pub mod serial;

pub trait Console {
    fn print(&mut self, s: &str) {
        for c in s.chars() {
            self.print_char(c);
        }
    }
    fn print_num(&mut self, n: &u64) {
        let mut buffer = [0u8; 50];
        self.print(n.numtoa_str(10, &mut buffer));
    }
    fn print_hex(&mut self, n: &u64) {
        self.print("0x");
        let mut buffer = [0u8; 50];
        self.print(n.numtoa_str(16, &mut buffer));
    }
    fn println(&mut self, s: &str) {
        self.print(s);
        self.print_char('\n');
    }
    fn print_char(&mut self, c: char);
}

pub enum Color {
    Red = 0xFF0000,
    Green = 0xFF00,
    Blue = 0xFF,
    Black = 0x0
}