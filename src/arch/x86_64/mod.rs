use x86::{io::{inb, outb}, time::rdtsc};

pub mod asm;
pub mod gdt;
pub mod irq;
pub mod serial;

pub fn detect_cpu_speed() {
    log::debug!("calculating approx cpu speed");
    let mut cycles_before = 0;
    let mut cycles_after = 0;
    let mut l: u32 = 0;
    let mut h: u32 = 0;
    interrupts::without(|| {
        unsafe {
            outb(0x43, 0x34);
            outb(0x40, 0);
            outb(0x40, 0);
            cycles_before = rdtsc();
            for _ in 0..0x1000 {}
            cycles_after = rdtsc();
            outb(0x43, 0x04);
            l = inb(0x40) as u32;
            h = inb(0x40) as u32;
        }
    });
    let ticks = 0x10000 - (h * 256 + l);
    let hz_speed = (cycles_after - cycles_before) * 1193180 / ticks as u64;
    log::info!("cpu speed: {} MHz", hz_speed as f32 / 1000000.0);
}
