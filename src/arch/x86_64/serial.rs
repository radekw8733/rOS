use super::asm::io_out;

pub struct SerialPort {
    addr: u16
}

impl SerialPort {
    pub fn get_first_port() -> SerialPort {
        SerialPort {
            addr: 0x3F8
        }
    }

    pub fn send_char(&mut self, c: char) {
        io_out(self.addr, c as u8);
    }
}