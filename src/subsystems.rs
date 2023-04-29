use spin::RwLock;

use crate::{terminal::fb::FramebufferConsole, bootboot::{_binary_font_psf_start, psf2_t}};
 
pub struct Subsystems {
    pub console: Option<FramebufferConsole>
}

pub static SUBSYSTEMS: RwLock<Subsystems> = RwLock::new(Subsystems {
    console: None
});

impl Subsystems {
    pub fn init(&mut self) {
        self.init_console();
    }

    pub fn init_console(&mut self) {
        let font = unsafe { (&_binary_font_psf_start as *const u64 as *const psf2_t).as_ref().unwrap() };
        self.console = Some(FramebufferConsole::new(font));
    }
}