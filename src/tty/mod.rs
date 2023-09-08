use crate::CONSOLE;

pub mod fb;

pub trait Console {
    fn write(&mut self, c: char);

    fn print(&mut self, s: &str) {
        for c in s.chars() {
            Self::write(self, c);
        }
    }

    fn println(&mut self, s: &str) {
        Self::print(self, s);
        Self::write(self, '\n');
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::tty::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    CONSOLE.lock().as_mut().write_fmt(args).unwrap();
}