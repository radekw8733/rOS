use spin::RwLock;
use x86::{
    io::{inb, outb},
    time::rdtsc,
};
use x86_64::{
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable}, paging::{Mapper, Page, PageTableFlags, Size4KiB, Translate}, tss::TaskStateSegment
    }, VirtAddr
};

use crate::kernel::mm::{phys_allocator::PHYS_ALLOCATOR, vm_manager::VM_MANAGER};

pub mod asm;
pub mod irq;
pub mod serial;

static mut KERNEL_STACK: [u8; 8192] = [0; 8192]; // needs to be mutable because of possible R/O paging flag set by bootloader
static GDT: RwLock<GlobalDescriptorTable> = RwLock::new(GlobalDescriptorTable::new());
static TSS: RwLock<TaskStateSegment> = RwLock::new(TaskStateSegment::new());

pub fn detect_cpu_speed() {
    log::debug!("calculating approx cpu speed");
    let mut cycles_before = 0;
    let mut cycles_after = 0;
    let mut l: u32 = 0;
    let mut h: u32 = 0;
    interrupts::without(|| unsafe {
        outb(0x43, 0x34);
        outb(0x40, 0);
        outb(0x40, 0);
        cycles_before = rdtsc();
        for _ in 0..0x1000 {}
        cycles_after = rdtsc();
        outb(0x43, 0x04);
        l = inb(0x40) as u32;
        h = inb(0x40) as u32;
    });
    let ticks = 0x10000 - (h * 256 + l);
    let hz_speed = (cycles_after - cycles_before) * 1193180 / ticks as u64;
    log::info!("cpu speed: {} MHz", hz_speed as f32 / 1000000.0);
}

pub fn prepare_userspace() {
    interrupts::without(|| {
        {
            let mut tss = TSS.write();
            unsafe {
                let stack_base_addr = KERNEL_STACK.as_ptr() as u64;
                let stack_ptr_addr = stack_base_addr + KERNEL_STACK.len() as u64;
                // set ESP0
                tss.privilege_stack_table[0] = VirtAddr::new(stack_ptr_addr);
            }
        }
        let mut gdt = GDT.write();
        gdt.append(Descriptor::kernel_code_segment());
        gdt.append(Descriptor::kernel_data_segment());
        gdt.append(Descriptor::user_code_segment());
        gdt.append(Descriptor::user_data_segment());
        unsafe {
            gdt.append(Descriptor::tss_segment_unchecked(TSS.as_mut_ptr()));
            gdt.load_unsafe();
            // reload TSS
            core::arch::asm!("mov ax, 0x28", "ltr ax");
        }
    });

    let frame = PHYS_ALLOCATOR.lock().get_mut().unwrap().alloc(1);
    let page = VM_MANAGER.lock().get_mut().unwrap().ident_map.translate(VirtAddr::new(frame.start as u64));
    unsafe {
        VM_MANAGER.lock().get_mut().unwrap().ident_map.update_flags(
            Page::<Size4KiB>::containing_address(VirtAddr::new(frame.start as u64)),
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE
        ).unwrap().flush();
    }
    let page = VM_MANAGER.lock().get_mut().unwrap().ident_map.translate(VirtAddr::new(frame.start as u64));

    let userspace_test_bin = frame.start as *mut [u8; 4];
    unsafe {
        *userspace_test_bin = [0xEB, 0, 0xEB, 0xFE]; // loop
    }

    unsafe {
        // enable SYSCALL/SYSRET and jump to userspace
        core::arch::asm!(
            "mov rcx, 0xc0000082",
            "wrmsr",
            "mov rcx, 0xc0000080",
            "rdmsr",
            "or eax, 1",
            "wrmsr",
            "mov rcx, 0xc0000081",
            "rdmsr",
            "mov edx, 0x00180008",
            "wrmsr",

            "mov rcx, r12", // function address to jump
            "mov r11, 0x202",
            "sysretq",
            in("r12") frame.start
        )
    }
}