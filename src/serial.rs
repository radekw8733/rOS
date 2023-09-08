use crate::assembly_macros::io_out;

pub struct SerialPort {
    addr: u16
}

impl SerialPort {
    pub fn get_first_port() -> SerialPort {
        let mut port = SerialPort {
            addr: 0x3F8
        };
        port.init();
        port
    }

    pub fn init(&mut self) {
        // not needed on BOOTBOOT
        // io_out(self.addr + 1, 0x00);
        // io_out(self.addr + 3, 0x80);
        // io_out(self.addr + 0, 0x00);
        // io_out(self.addr + 1, 0x00);
        // io_out(self.addr + 3, 0x03);
        // io_out(self.addr + 2, 0xC7);
        // io_out(self.addr + 4, 0x03);
    }

    pub fn send_char(&mut self, c: char) {
        io_out(self.addr, c as u8);
    }
}