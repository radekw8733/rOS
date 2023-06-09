use spin::RwLock;
use crate::{terminal::{CONSOLE, ConsoleTypes}, memory::paging::{Paging, PagingInterface}, println};
 
pub struct Subsystems {
    pub paging: Option<Paging>
}

pub static SUBSYSTEMS: RwLock<Subsystems> = RwLock::new(Subsystems {
    paging: None
});

impl Subsystems {
    pub fn init(&mut self) {
        self.init_paging();
        self.init_console();
    }

    fn init_console(&mut self) {
        CONSOLE.lock().init_console(ConsoleTypes::All);
        println!("initializing...");
    }

    fn init_paging(&mut self) {
        println!("initializing virtual memory");
        self.paging = Some(Paging::new());
        println!("virtual memory ready");

        println!("allocating heap memory for kernel");
        println!("{}", self.paging.as_mut().unwrap().allocate_pages_for_kernel().unwrap());
        println!("space for heap ready");
    }
}