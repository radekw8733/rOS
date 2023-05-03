use crate::serial::x86::SerialPort;

use super::Console;

pub struct SerialConsole {
    port: SerialPort
}

impl SerialConsole {
    pub fn new() -> SerialConsole {
        SerialConsole { port: SerialPort::get_first_port() }
    }
}

impl Console for SerialConsole {
    fn print_char(&mut self, c: char) {
        self.port.send_char(c);
    }
}