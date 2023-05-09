#![allow(dead_code)]
use spin::Mutex;

use crate::assembly_macros::{io_out, io_in, io_delay};

const PIC1: u16 = 0x20;
const PIC2: u16 = 0xA0;
const PIC1_COMMAND: u16 = PIC1;
const PIC2_COMMAND: u16 = PIC2;
const PIC1_DATA: u16 = PIC1 + 1;
const PIC2_DATA: u16 = PIC2 + 1;
const PIC_EOI: u8 = 0x20;
pub const PIC1_INT_OFFSET: u8 = 0x20;
pub const PIC2_INT_OFFSET: u8 = 0x28;

const ICW1_ICW4: u8 = 0x01;
const ICW1_SINGLE: u8 = 0x02;
const ICW1_INTERVAL4: u8 = 0x04;
const ICW1_LEVEL: u8 = 0x08;
const ICW1_INIT: u8 = 0x10;

const ICW4_8086: u8 = 0x01;
const ICW4_AUTO: u8 = 0x02;
const ICW4_BUF_SLAVEL: u8 = 0x08;
const ICW4_BUS_MASTER: u8 = 0x0C;
const ICW4_SFNM: u8 = 0x10;

pub struct PICType;

pub static PIC: Mutex<PICType> = Mutex::new(PICType);

impl PICType {
    pub fn init(&self) {
        let mask1 = io_in(PIC1_DATA);
        let mask2 = io_in(PIC2_DATA);

        io_out(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);
        io_delay();
        io_out(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);
        io_delay();
        io_out(PIC1_DATA, PIC1_INT_OFFSET);
        io_delay();
        io_out(PIC2_DATA, PIC2_INT_OFFSET);
        io_delay();
        io_out(PIC1_DATA, 4);
        io_delay();
        io_out(PIC2_DATA, 2);
        io_delay();
        io_out(PIC1_DATA, ICW4_8086);
        io_delay();
        io_out(PIC2_DATA, ICW4_8086);

        io_out(PIC1_DATA, mask1);
        io_out(PIC2_DATA, mask2);
    }

    pub fn eoi(&self, irq: u8) {
        if irq >= 8 {
            io_out(PIC2_COMMAND, PIC_EOI);
        }
        io_out(PIC1, PIC_EOI);
    }

    pub fn mask(&self, irq: u8) {
        let mut port = 0;
        let mut irq = irq;
        let mut mask = 0;
        if irq < 8 {
            port = PIC1_DATA;
        }
        else {
            port = PIC2_DATA;
            irq -= 8;
        }
        mask = io_in(port);
        mask = mask | 1 << irq;
        io_out(port, mask);
    }

    pub fn unmask(&self, irq: u8) {
        let mut port = 0;
        let mut irq = irq;
        let mut mask = 0;
        if irq < 8 {
            port = PIC1_DATA;
        }
        else {
            port = PIC2_DATA;
            irq -= 8;
        }
        mask = io_in(port);
        mask = mask & !(1 << irq);
        io_out(port, mask);
    }
}