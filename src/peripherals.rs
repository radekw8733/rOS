use spin::RwLock;
use x86_64::structures::{gdt::GlobalDescriptorTable, idt::InterruptDescriptorTable};

use crate::{terminal::{shell::Shell}, bootboot::{BOOTBOOT_INFO, BOOTBOOT}, keyboard::ps2::PS2ControllerType, interrupts::{idt::IDT, pic::PIC}, timer::pit::PIT, graphics::framebuffer::Framebuffer};

pub struct Peripherals {
    pub framebuffer: Option<Framebuffer>,
    pub shell: Option<Shell>,
    #[cfg(target_arch = "x86_64")]
    pub gdt: Option<GlobalDescriptorTable>,
    #[cfg(target_arch = "x86_64")]
    pub idt: Option<&'static InterruptDescriptorTable>,
    #[cfg(target_arch = "x86_64")]
    pub ps2_controller: Option<PS2ControllerType>,
    #[cfg(target_arch = "x86_64")]
    pub pit: Option<PIT>,
    // pub serial_port: Option<SerialPort>
}

// initialize on start rather than statically
pub static PERIPHERALS: RwLock<Peripherals> = RwLock::new(Peripherals {
    framebuffer: None,
    shell: None,
    #[cfg(target_arch = "x86_64")]
    gdt: None,
    #[cfg(target_arch = "x86_64")]
    idt: None,
    #[cfg(target_arch = "x86_64")]
    ps2_controller: None,
    #[cfg(target_arch = "x86_64")]
    pit: None,
    // serial_port: None
});

impl Peripherals {
    pub fn init(&mut self) {
        // self.init_serial_port();
        #[cfg(feature = "bootboot")]
        self.init_framebuffer();
        #[cfg(target_arch = "x86_64")] {
            // self.init_gdt();
            self.init_shell();
            self.init_pic(); // works on qemu
            self.init_ps2();
            // self.init_pit();
            self.init_idt();
        }
    }

    fn init_framebuffer(&mut self) {
        if self.framebuffer.is_none() {
            let bootboot_info = unsafe { &*(BOOTBOOT_INFO as *const BOOTBOOT) };

            self.framebuffer = Some(Framebuffer { 
                fb_addr: bootboot_info.fb_ptr as usize,
                fb_size: bootboot_info.fb_size,
                fb_width: bootboot_info.fb_width,
                fb_height: bootboot_info.fb_height,
                fb_scanline: bootboot_info.fb_scanline
            });
        }
    }

    fn init_shell(&mut self) {
        self.shell = Some(Shell);
    }

    #[cfg(target_arch = "x86_64")]
    fn init_gdt(&mut self) {
        use x86_64::structures::gdt::Descriptor;

        let mut gdt = GlobalDescriptorTable::new();
        gdt.add_entry(Descriptor::kernel_code_segment());
        gdt.add_entry(Descriptor::kernel_data_segment());

        self.gdt.replace(gdt);
        unsafe { self.gdt.as_mut().unwrap().load_unsafe() };
    }

    #[cfg(target_arch = "x86_64")]
    fn init_ps2(&mut self) {
        self.ps2_controller = Some(PS2ControllerType::new());
    }

    #[cfg(target_arch = "x86_64")]
    fn init_idt(&mut self) {
        IDT.load();
    }

    #[cfg(target_arch = "x86_64")]
    fn init_pit(&mut self) {
        self.pit = Some(PIT::new())
    }

    #[cfg(target_arch = "x86_64")]
    fn init_pic(&mut self) {
        PIC.lock().init()
    }

    // fn init_serial_port(&mut self) {
    //     if self.serial_port.is_none() {
    //         self.serial_port = Some(SerialPort::get_first_port());
    //     }
    // }
}