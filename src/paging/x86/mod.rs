use x86::bits64::paging::{PML4Entry, PDPTEntry, PDEntry, PTEntry};

use crate::assembly_macros::get_pd_addr;

const PAGE_TABLES: usize = 512;

#[repr(align(4096))]
pub struct PML4Table {
    pub entries: [PDPTEntry; PAGE_TABLES]
}

#[repr(align(4096))]
pub struct PDPTTable {
    pub entries: [PDEntry; PAGE_TABLES]
}

#[repr(align(4096))]
pub struct PDTable {
    pub entries: [PTEntry; PAGE_TABLES]
}

pub struct Paging {
    pub pml4: &'static PML4Entry,
    pub tables: &'static PML4Table
}

impl Paging {
    pub fn new() -> Paging {
        let root_page_addr = get_pd_addr();
        let pml4 = unsafe { &*(root_page_addr as *mut PML4Entry) };
        let pml4_table = unsafe { &*(pml4.address().as_usize() as *mut PML4Table) };

        Paging {
            pml4,
            tables: pml4_table,
        }
    }
}