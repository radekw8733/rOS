use spin::RwLock;
use crate::{terminal::{fb::FramebufferConsole, Console, serial::SerialConsole}, bootboot::{_binary_font_psf_start, psf2_t}, memory::paging::x86::Paging};
 
pub struct Subsystems {
    pub console: Option<FramebufferConsole>,
    pub paging: Option<Paging>
}

pub static SUBSYSTEMS: RwLock<Subsystems> = RwLock::new(Subsystems {
    console: None,
    paging: None
});

impl Subsystems {
    pub fn init(&mut self) {
        self.init_console();
        self.init_paging();
    }

    fn init_console(&mut self) {
        let font = unsafe { (&_binary_font_psf_start as *const u64 as *const psf2_t).as_ref().unwrap() };
        self.console = Some(FramebufferConsole::new(font));
        self.console.as_mut().unwrap().println("initializing...\n");

        // self.console = Some(SerialConsole::new());
    }

    fn init_paging(&mut self) {
        let console = self.console.as_mut().unwrap();
        console.println("initializing virtual memory");
        self.paging = Some(Paging::new());
        console.println("virtual memory ready");

        console.println("allocating heap memory for kernel");
        console.println(self.paging.as_mut().unwrap().allocate_pages_for_kernel().unwrap());
        console.println("space for heap ready");
    }
}