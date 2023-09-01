use spin::RwLock;
use x86_64::structures::gdt::GlobalDescriptorTable;

static mut GDT: RwLock<GlobalDescriptorTable> = RwLock::new(GlobalDescriptorTable::new());

pub fn load_gdt() {
    use x86_64::structures::gdt::Descriptor;

    let mut gdt = GlobalDescriptorTable::new();
    gdt.add_entry(Descriptor::kernel_code_segment());
    gdt.add_entry(Descriptor::kernel_data_segment());

    unsafe {
        GDT = RwLock::new(gdt);
        GDT.write().load_unsafe();
    }
}