.code64

.section .text.boot
.global _start
.type _start, @function
_start:
    mov eax, 1
    cpuid
    shr ebx, 24
    cmp [bootboot + 0xC], bx
    jne .ap
    call kmain
.ap:
    call ap