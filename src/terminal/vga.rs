static mut VGA_BUFFER: *mut u16 = 0xB8000 as *mut u16;
static mut VGA_CURSOR_X: u8 = 0;
static mut VGA_CURSOR_Y: u8 = 0;

const VGA_WIDTH: u8 = 80;
const VGA_HEIGHT: u8 = 25;

pub fn print(s: &str) {
    for c in s.chars() {
        put_char(c as u8);
    }
}

fn put_char(c: u8) {
    if c == b'\n' {
        unsafe {
            VGA_CURSOR_X = 0;
            VGA_CURSOR_Y += 1;
        }
    }
    else {
        let c = char_to_vga_entry(c as u16);

        unsafe {
            *VGA_BUFFER.offset((VGA_CURSOR_Y * VGA_WIDTH + VGA_CURSOR_X) as isize) = c;
        }
    }

    unsafe {
        VGA_CURSOR_X += 1;

        if VGA_CURSOR_X == VGA_WIDTH {
            VGA_CURSOR_X = 0;
            if VGA_CURSOR_Y == VGA_HEIGHT {
                VGA_CURSOR_Y = 0;
            }
        }
    }
}

fn char_to_vga_entry(c: u16) -> u16 {
    c | 15u16 << 8
}