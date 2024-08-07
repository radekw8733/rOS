use spin::{Lazy, RwLock};
use x86_64::structures::gdt::GlobalDescriptorTable;

static GDT: Lazy<RwLock<GlobalDescriptorTable>> = Lazy::new(|| {
    use x86_64::structures::gdt::Descriptor;

    let mut gdt = GlobalDescriptorTable::new();
    gdt.append(Descriptor::kernel_code_segment());
    gdt.append(Descriptor::kernel_data_segment());
    RwLock::new(gdt)
});

pub fn _load_gdt() {
    unsafe { GDT.read().load_unsafe() }
}