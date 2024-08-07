#![allow(unused_assignments)]

use core::arch::asm;

// returns EBX, EDX, ECX
pub fn _cpuid(op: u32) -> (u32, u32, u32) {
    let mut ebx: u32 = 0;
    let mut edx: u32 = 0;
    let mut ecx: u32 = 0;
    
    unsafe {
        asm!(
            "mov eax, {0:e}",
            "cpuid",
            "mov {1:e}, ebx",
            "mov {2:e}, edx",
            "mov {3:e}, ecx",
            in(reg) op,
            out(reg) ebx,
            out(reg) edx,
            out(reg) ecx
        )
    }

    (ebx, edx, ecx)
}

// Read 8bit value from IO port
#[inline]
pub fn io_in(src: u16) -> u8 {
    let mut data: u8 = 0;

    unsafe {
        asm!(
            "in al, dx",
            in("dx") src,
            out("al") data
        )
    }

    data
}

// Write 8bit value to IO port
#[inline]
pub fn io_out(dest: u16, data: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") dest,
            in("al") data
        )
    }
}

// Read 32bit value from IO port
#[inline]
pub fn io_inl(src: u16) -> u32 {
    let mut data: u32 = 0;

    unsafe {
        asm!(
            "in eax, dx",
            in("dx") src,
            out("eax") data
        )
    }

    data
}

// Write 32bit value to IO port
#[inline]
pub fn io_outl(dest: u16, data: u32) {
    unsafe {
        asm!(
            "out dx, eax",
            in("dx") dest,
            in("eax") data
        )
    }
}

#[inline]
pub fn io_delay() {
    io_out(0x80, 0)
}