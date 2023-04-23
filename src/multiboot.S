.code32
.set ALIGN,     1<<0
.set MEMINFO,   1<<1
.set FLAGS,     ALIGN | MEMINFO
.set MAGIC,     0x1BADB002
.set CHECKSUM,  -(MAGIC + FLAGS)

.section .multiboot
.align 4
.long MAGIC
.long FLAGS
.long CHECKSUM

.section .bss
.lcomm pml_table, 4096
.lcomm pdp_table, 4096
.lcomm pd_table, 4096
.lcomm pt_table, 4096
.lcomm stack_bottom, 64
.align 4096
stack_top:

.section .text
.global _start
.type _start, @function
_start:
    mov esp, stack_top # initialize stack

    call page_table_setup
    call page_enable
    call kmain

    cli
1:  hlt
    jmp 1b

page_table_setup:
#  mapping PML (4) to PDP (3);
    mov eax, pdp_table
    or eax, 0b11
    mov [pml_table], eax

#  mapping PDP (3) to PD (2)
    mov eax, pd_table
    or  eax, 0b11

#  mapping PD (2) to PT (1)
    or eax, 0b10000011
    mov [pd_table * 8], eax

page_enable:
#  load PML to CR3
    mov eax, pml_table
    mov cr3, eax

#  enable PAE
    mov eax, cr4
    or eax, 1 << 5

#  set long mode bit
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

# enable paging
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

.size _start, . - _start
