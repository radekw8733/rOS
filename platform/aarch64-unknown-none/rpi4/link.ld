ENTRY(_start)

SECTIONS {
    . = 0x80000;

    .text : ALIGN(4K) {
        KEEP(*(.text.boot))
        *(.text)
    }

    .rodata : ALIGN(4K) {
        *(.rodata)
    }

    .data : ALIGN(4K) {
        *(.data)
    }

    __bss_start = .;
    .bss : ALIGN(4K) {
        bss = .;
        *(.bss)
    }
    __bss_end = .;
    __bss_size = __bss_end - __bss_start;
    __end = .;
}