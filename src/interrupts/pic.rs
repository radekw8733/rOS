#![allow(dead_code)]
#![allow(unused_assignments)]
use spin::Mutex;

use crate::arch::{io_out, io_in, io_delay};

const PIC1: u16 = 0x20;
const PIC2: u16 = 0xA0;
const PIC1_COMMAND: u16 = PIC1;
const PIC2_COMMAND: u16 = PIC2;
const PIC1_DATA: u16 = PIC1 + 1;
const PIC2_DATA: u16 = PIC2 + 1;
const PIC_EOI: u8 = 0x20; // end of interrupt
pub const PIC1_INT_OFFSET: u8 = 0x20; // master PIC range 0x20..0x2F
pub const PIC2_INT_OFFSET: u8 = 0x28; // slave PIC range 0x30..0x3F

const ICW1_ICW4: u8 = 0x01; // ICW4 present
const ICW1_SINGLE: u8 = 0x02; // single PIC mode
const ICW1_INTERVAL4: u8 = 0x04; 
const ICW1_LEVEL: u8 = 0x08; // edge trigger mode
const ICW1_INIT: u8 = 0x10; // init byte

const ICW4_8086: u8 = 0x01; // 8086 mode EOI
const ICW4_AUTO: u8 = 0x02; // normal mode EOI
const ICW4_BUF_SLAVE: u8 = 0x08; // slave PIC buffered
const ICW4_BUS_MASTER: u8 = 0x0C; // master PIC buffered
const ICW4_SFNM: u8 = 0x10; // special fully nested

#[repr(C)]
#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum PICInterrupt {
    PIT = 0,
    PS2Keyboard = 1,
}

impl PICInterrupt {
    pub fn to_idt_entry_index(&self) -> usize {
        (*self as usize) + PIC1_INT_OFFSET as usize
    }
}

pub struct PICType;

pub static PIC: Mutex<PICType> = Mutex::new(PICType);

impl PICType {
    pub fn init(&self) {
        let mask1 = io_in(PIC1_DATA);
        let mask2 = io_in(PIC2_DATA);

        // ICW1
        io_out(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);
        io_delay();
        io_out(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);
        io_delay();
        
        // ICW2
        io_out(PIC1_DATA, PIC1_INT_OFFSET);
        io_delay();
        io_out(PIC2_DATA, PIC2_INT_OFFSET);
        io_delay();

        // ICW3
        io_out(PIC1_DATA, 4);
        io_delay();
        io_out(PIC2_DATA, 2);
        io_delay();

        // ICW4
        io_out(PIC1_DATA, ICW4_8086);
        io_delay();
        io_out(PIC2_DATA, ICW4_8086);

        // masks
        io_out(PIC1_DATA, mask1);
        io_out(PIC2_DATA, mask2);
    }

    // accept next interrupts from PIC
    pub fn eoi(&self, irq: PICInterrupt) {
        let irq = irq as u8;
        if irq >= 8 {
            io_out(PIC2_COMMAND, PIC_EOI);
        }
        io_out(PIC1_COMMAND, PIC_EOI);
    }

    pub fn mask(&self, irq: PICInterrupt) {
        let mut port = 0;
        let mut irq = irq as u8;
        let mut mask = 0;
        if irq < 8 {
            port = PIC1_DATA;
        }
        else {
            port = PIC2_DATA;
            irq -= 8;
        }
        mask = io_in(port);
        mask |= 1 << irq;
        io_out(port, mask);
    }

    pub fn unmask(&self, irq: PICInterrupt) {
        let mut port = 0;
        let mut irq = irq as u8;
        let mut mask = 0;
        if irq < 8 {
            port = PIC1_DATA;
        }
        else {
            port = PIC2_DATA;
            irq -= 8;
        }
        mask = io_in(port);
        mask &= !(1 << irq);
        io_out(port, mask);
    }
}