use core::fmt::Write;

use alloc::{boxed::Box, format, string::{String, ToString}, vec::Vec};
use ansi_rgb::Foreground as _;
use rgb::RGB8;
use spin::Mutex;

pub static LOGGER: Mutex<Logger> = Mutex::new(Logger::new());

pub struct Logger {
    early_mode: bool,
    early_buffer: [char; 10000],
    early_counter: usize,
    consoles: Vec<Box<dyn Console + Send>>,
    buffer: Vec<String>,
}

impl Logger {
    pub const fn new() -> Self {
        Self {
            early_mode: true,
            early_buffer: ['\0'; 10000],
            early_counter: 0,
            consoles: Vec::new(),
            buffer: Vec::new(),
        }
    }

    pub fn add_console(&mut self, console: Box<dyn Console + Send>) {
        self.consoles.push(console);
    }

    pub fn switch_to_allocated_mode(&mut self) {
        let cont_r = self.early_buffer.iter().collect::<String>();
        let cont = cont_r.split('\n').collect::<Vec<&str>>();
        for console in &mut self.consoles {
            for line in &cont {
                console.write_str(&line.chars()
                    .filter(|c| *c != '\0')
                    .collect::<String>());
                console.write_char('\n').unwrap();
            }
        }
        self.early_mode = false;
        log::set_logger(&LogAdapter).unwrap();
        log::set_max_level(log::LevelFilter::Trace);
    }
}

impl core::fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self.early_mode {
            true => {
                for c in s.chars() {
                    self.early_buffer[self.early_counter] = c;
                    self.early_counter += 1;
                }
            },
            false => {
                self.buffer.push(s.to_string());
                for console in &mut self.consoles {
                    console.write_str(s);
                }
            }
        }
        Ok(())
    }
}

pub struct LogAdapter;

impl log::Log for LogAdapter {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let sign_color = RGB8::new(100, 100, 100);
        let target_color = RGB8::new(170, 170, 170);
        let msg_color = RGB8::new(255, 255, 255);

        let level_color = match record.level() {
            log::Level::Error => RGB8::new(255, 0, 0),
            log::Level::Warn => RGB8::new(255, 165, 0),
            log::Level::Info => RGB8::new(50, 50, 255),
            log::Level::Debug => RGB8::new(0, 75, 140),
            log::Level::Trace => RGB8::new(100, 100, 100),
        };

        LOGGER.lock().write_str(&format!(
            "{}{level} {target}{} {msg}\n", '['.fg(sign_color), "]:".fg(sign_color),
            level = record.level().as_str().fg(level_color),
            target = record.target().fg(target_color),
            msg = record.args().fg(msg_color)
        )).unwrap();
    }

    fn flush(&self) {}
}


pub trait Console {
    fn write(&mut self, c: char);

    fn write_str(&mut self, s: &str) {
        for c in s.chars() {
            Self::write(self, c);
        }
    }
}

impl core::fmt::Write for (dyn Console + Send + 'static) {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (crate::kernel::log::_print(format_args!($($arg)*), false));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => (crate::kernel::log::_print(format_args!($($arg)*), true));
}

#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    ($($arg:tt)*) => ($crate::eprint!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments, emergency: bool) {
    use core::fmt::Write;
    if emergency {
        unsafe { LOGGER.force_unlock() };
    }
    LOGGER.lock().write_fmt(args).unwrap()
}