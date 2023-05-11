use super::GenericConsole;

const VGA_BUFFER: usize = 0xB8000;

const VGA_WIDTH: u8 = 80;
const VGA_HEIGHT: u8 = 25;

pub struct VgaConsole {
    charpos_x: u8,
    charpos_y: u8,
}

impl GenericConsole for VgaConsole {
    fn print_char(&mut self, c: char) {
        self.put_char(c as u8);
    }
}

impl VgaConsole {
    pub fn new() -> VgaConsole {
        VgaConsole {
            charpos_x: 0, 
            charpos_y: 0
        }
    }

    pub fn scroll(&mut self) {
        self.scrolls(1);
    }

    pub fn scrolls(&mut self, n: usize) {
        let vram = unsafe { &mut *(VGA_BUFFER as *mut [[u16; VGA_WIDTH as usize]; VGA_HEIGHT as usize]) };

        for row_index in 0..n {
            let row = &mut vram[row_index];
            row.fill(Self::char_to_vga_entry(' ' as u16));
        }

        for y in n..VGA_HEIGHT as usize {
            for x in 0..VGA_WIDTH as usize {
                let src = vram[y][x];
                let dst = &mut vram[y - n][x];
                *dst = src;
            }
        }
        vram[VGA_HEIGHT as usize - 1].fill(Self::char_to_vga_entry(' ' as u16));
    }

    fn put_char(&mut self, c: u8) {
        if c == b'\n' {
            self.charpos_x = 0;
            if self.charpos_y < VGA_HEIGHT - 1 {
                self.charpos_y += 1;
            }
            else {
                self.scroll();
            }
        }
        else {
            let c = VgaConsole::char_to_vga_entry(c as u16);

            if self.charpos_x >= VGA_WIDTH {
                self.charpos_y += 1;
                self.charpos_x = 1;
            }

            let addr = VGA_BUFFER + ((self.charpos_y as u16 * VGA_WIDTH as u16 + self.charpos_x as u16) * 2) as usize;
            let ptr = addr as *mut u16;
            unsafe { *ptr = c };

            self.charpos_x += 1;
        }
    }
    
    fn char_to_vga_entry(c: u16) -> u16 {
        c | 15u16 << 8
    }
}