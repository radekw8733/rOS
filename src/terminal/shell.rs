use lazy_static::lazy_static;
use pc_keyboard::{layouts, ScancodeSet1, Keyboard};
use spin::Mutex;

use crate::{print, println};

pub struct Shell;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
        Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, pc_keyboard::HandleControl::Ignore)
    );
}

static SHELL: Shell = Shell;

impl Shell {
    pub fn main_loop() {
        println!();
        println!("Welcome to rOS v{}!", crate::VERSION);
        Self::print_sign();
    }

    fn print_sign() {
        print!("> ");
    }

    pub fn pass_key(scan_code: u8) {
        let mut keyboard = KEYBOARD.lock();

        if let Ok(Some(key_event)) = keyboard.add_byte(scan_code) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    pc_keyboard::DecodedKey::RawKey(key) => {
                        print!("{:?}", key);
                    }
                    pc_keyboard::DecodedKey::Unicode(c) => {
                        print!("{}", c);
                        if c == '\n' {
                            Self::print_sign();
                        }
                    }
                }
            }
        }
    }
}