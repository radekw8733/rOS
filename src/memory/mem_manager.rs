use linked_list_allocator::LockedHeap;
use x86_64::VirtAddr;

use super::paging::{x86::{X86Paging, PDTable}, PagingInterface};

#[global_allocator]
static KERNEL_HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

pub struct MemoryManager {
    paging_manager: X86Paging,
    phys_mem_offset: u64
}

impl MemoryManager {
    pub fn new() -> MemoryManager {
        MemoryManager {
            paging_manager: X86Paging::new(),
            phys_mem_offset: 0
        }
    }

    pub fn set_phys_mem_offset(&mut self, phys_mem_offset: u64) {
        self.phys_mem_offset = phys_mem_offset;
        self.paging_manager.init(VirtAddr::new(phys_mem_offset));
    }

    pub fn allocate_pages_for_kernel(&mut self) {
        let kernel_heap_addr = 0x0000010000000000usize;
        let kernel_heap_phys_addr = 0x200000;
        let kernel_heap_pages_count = 10;
        self.paging_manager.map(kernel_heap_addr, kernel_heap_phys_addr, kernel_heap_pages_count);
        // unsafe {
        //     KERNEL_HEAP_ALLOCATOR.lock().init(kernel_heap_addr as *mut u8, 2048 * kernel_heap_pages_count);
        // }
    }

    pub fn get_kernel_pages(&self) -> Option<&&mut PDTable> {
        self.paging_manager.kmem.as_ref()
    }
}