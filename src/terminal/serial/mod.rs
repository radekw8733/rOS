use crate::serial::x86::SerialPort;

use super::GenericConsole;

pub struct SerialConsole {
    port: SerialPort
}

impl SerialConsole {
    pub fn new() -> SerialConsole {
        SerialConsole { port: SerialPort::get_first_port() }
    }
}

impl GenericConsole for SerialConsole {
    fn print_char(&mut self, c: char) {
        self.port = SerialPort::get_first_port();
        self.port.send_char(c);
    }
}