use core::cell::OnceCell;

use alloc::vec::Vec;
use spin::Mutex;
use x86_64::structures::paging::OffsetPageTable;

use super::MMapEntry;

pub struct VirtualMemoryManager<'a> {
    pub ident_map: OffsetPageTable<'a>,
    pub regions: Vec<MMapEntry>
}

impl<'a> VirtualMemoryManager<'a> {
    pub fn new(ident_map: OffsetPageTable<'a>) -> Self {
        Self {
            ident_map,
            regions: Vec::new()
        }
    }
}

pub static VM_MANAGER: Mutex<OnceCell<VirtualMemoryManager>> = Mutex::new(OnceCell::new());

// TODO: implement loading executables into memory when ELF loader is finished