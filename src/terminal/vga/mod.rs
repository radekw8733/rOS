use crate::terminal::Console;

static mut VGA_BUFFER: *mut u16 = 0xB8000 as *mut u16;
static mut vga_cursor_x: u8 = 0;
static mut vga_cursor_y: u8 = 0;

const VGA_WIDTH: u8 = 80;
const VGA_HEIGHT: u8 = 25;

pub struct VgaConsole;

impl Console for VgaConsole {
    fn println(&mut self, s: &str) {
        for c in s.chars() {
            self.put_char(c as u8);
        }
    }
}
impl VgaConsole {
    fn put_char(&mut self, c: u8) {
        if c == b'\n' {
            unsafe {
                vga_cursor_x = 0;
                vga_cursor_y += 1;
            }
        }
        else {
            let c = VgaConsole::char_to_vga_entry(c as u16);
    
            unsafe {
                *VGA_BUFFER.offset((vga_cursor_y * VGA_WIDTH + vga_cursor_x) as isize) = c;
            }
        }

        unsafe {
            vga_cursor_x += 1;

            if vga_cursor_x == VGA_WIDTH {
                vga_cursor_x = 0;
                if vga_cursor_y == VGA_HEIGHT {
                    vga_cursor_y = 0;
                }
            }
        }
    }
    
    fn char_to_vga_entry(c: u16) -> u16 {
        c | 15u16 << 8
    }
}