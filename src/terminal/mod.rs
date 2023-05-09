#![allow(dead_code)]

use lazy_static::lazy_static;
use numtoa::NumToA;
use spin::Mutex;

use crate::{terminal::fb::FramebufferConsole, bootboot::{_binary_font_psf_start, psf2_t}};

use self::serial::SerialConsole;
pub mod fb;
pub mod serial;
pub mod shell;

pub trait GenericConsole {
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

pub struct Console {
    pub fbcon: Option<FramebufferConsole>,
    serialcon: Option<SerialConsole>,
}

impl GenericConsole for Console {
    fn print_char(&mut self, c: char) {
        if self.fbcon.is_some() {
            self.fbcon.as_mut().unwrap().print_char(c);
        }
        if self.serialcon.is_some() {
            self.serialcon.as_mut().unwrap().print_char(c);
        }
    }
}

impl core::fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}

impl Console {
    pub fn init_console(&mut self, cons: ConsoleTypes) {
        match cons {
            ConsoleTypes::Framebuffer => self.init_fbcon(),
            ConsoleTypes::Serial => self.init_serialcon(),
            ConsoleTypes::All => {
                self.init_fbcon(); self.init_serialcon();
            }
        }
    }

    fn init_fbcon(&mut self) {
        self.fbcon = Some(FramebufferConsole::new(unsafe { (&_binary_font_psf_start as *const u64 as *const psf2_t).as_ref().unwrap() }));
    }

    fn init_serialcon(&mut self) {
        self.serialcon = Some(SerialConsole::new())
    }
}

lazy_static! {
    pub static ref CONSOLE: Mutex<Console> = Mutex::new(Console {
        fbcon: None,
        serialcon: None
    });
}

pub enum Color {
    Red = 0xFF0000,
    Green = 0xFF00,
    Blue = 0xFF,
    Black = 0x0
}

pub enum ConsoleTypes {
    All,
    Serial,
    Framebuffer
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::terminal::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    CONSOLE.lock().write_fmt(args).unwrap();
}