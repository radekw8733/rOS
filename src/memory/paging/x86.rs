use core::mem::size_of;

use x86::{current::paging::{PML4Entry, PDPTEntry, PDEntry, PAGE_SIZE_ENTRIES, pml4_index, VAddr, PAddr, pdpt_index, pd_index, PDFlags, PDPTFlags, PML4Flags}, tlb::flush};

use crate::assembly_macros::{get_pd_addr, enable_large_pages};

const KERNEL_HEAP_PAGE: usize = 0xFFFFFF0000000000;
const KERNEL_HEAP_PAGE_COUNT: usize = 10; // 2 MB * 10 = 20 MB
const KERNEL_PAGE_TABLE_ADDR: usize = 0x2000000;

#[repr(align(4096))]
pub struct RootTable {
    pub entries: [PML4Entry; PAGE_SIZE_ENTRIES]
}
#[repr(align(4096))]
pub struct PDPTTable {
    pub entries: [PDPTEntry; PAGE_SIZE_ENTRIES]
}

#[repr(align(4096))]
pub struct PDTable {
    pub entries: [PDEntry; PAGE_SIZE_ENTRIES]
}

pub struct Paging {
    pub tables: &'static mut RootTable,
    pub kmem: Option<&'static mut PDTable>
}

impl Paging {
    pub fn new() -> Paging {
        let root_page_addr = get_pd_addr();
        let tables = unsafe { &mut *(root_page_addr as *mut RootTable) };

        Paging {
            tables,
            kmem: None,
        }
    }

    pub fn allocate_pages_for_kernel(&mut self) -> Result<&'static str, &'static str> {
        enable_large_pages();
        self.map(VAddr::from_usize(KERNEL_HEAP_PAGE), PAddr::from(0x200000));

        self.test_space()
    }

    pub fn map(&mut self, virt: VAddr, phys: PAddr) {
        let pml4_index = pml4_index(virt);
        let pdpt_index = pdpt_index(virt);
        let pd_index = pd_index(virt);

        let pml4 = unsafe { &mut *(&mut self.tables.entries[pml4_index] as *mut PML4Entry) };
        if !pml4.is_present() {
            *pml4 = PML4Entry::new(PAddr::from(KERNEL_PAGE_TABLE_ADDR), PML4Flags::P | PML4Flags::RW);
        }

        let table = unsafe { &mut *(pml4.address().as_usize() as *mut PDPTTable) };
        for x in &mut table.entries  {
            x.0 = 0;
        }

        let pdpt = unsafe { &mut *((pml4.address().as_usize() + size_of::<u64>() * pdpt_index) as *mut PDPTEntry) };

        let pds_addr = pml4.address().as_usize() + size_of::<PDPTEntry>() * 512 + (size_of::<u64>() * pd_index);
        *pdpt = PDPTEntry::new(
            PAddr::from(pds_addr),
            PDPTFlags::P | PDPTFlags::RW);

        for x in 0..KERNEL_HEAP_PAGE_COUNT {
            let pd = unsafe { &mut *((pds_addr + (x * size_of::<u64>())) as *mut PDEntry) };
            *pd = PDEntry::new(phys + x * 0x200000, PDFlags::P | PDFlags::RW | PDFlags::PS);
        }
        for x in KERNEL_HEAP_PAGE_COUNT..512 {
            let pd = unsafe { &mut *((pds_addr + (x * size_of::<u64>())) as *mut PDEntry) };
            pd.0 = 0;
        }

        unsafe { self.kmem = Some(&mut *(pds_addr as *mut PDTable)) };

        unsafe { flush(virt.as_usize()) };
    }

    fn test_space(&mut self) -> Result<&'static str, &'static str> {
        let test = 139;
        let addr = KERNEL_HEAP_PAGE as *mut u64;
        unsafe { *addr = test };
        if test == unsafe { *addr } {
            Ok("heap space test successful")
        }
        else {
            Err("invalid heap pages!")
        }
    }
}