#![allow(unused_assignments)]

use core::arch::asm;

// returns EBX, EDX, ECX
pub fn cpuid(op: u32) -> (u32, u32, u32) {
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

pub fn get_pd_addr() -> usize {
    let mut addr: usize = 0;

    unsafe {
        asm!(
            "mov {0}, cr3",
            out(reg) addr
        )
    }

    addr
}

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

