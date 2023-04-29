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