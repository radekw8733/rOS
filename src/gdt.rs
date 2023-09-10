use lazy_static::lazy_static;
use spin::RwLock;
use x86_64::structures::gdt::GlobalDescriptorTable;

lazy_static! {
    static ref GDT: RwLock<GlobalDescriptorTable> = RwLock::new({
        use x86_64::structures::gdt::Descriptor;

        let mut gdt = GlobalDescriptorTable::new();
        gdt.add_entry(Descriptor::kernel_code_segment());
        gdt.add_entry(Descriptor::kernel_data_segment());
        gdt
    });
}

pub fn _load_gdt() {
    unsafe { GDT.read().load_unsafe() }
}