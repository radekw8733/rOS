use core::{alloc::{GlobalAlloc, Layout}, cell::OnceCell, ptr::NonNull};

use alloc::vec::Vec;
use buddy_system_allocator::LockedHeap;
use spin::Mutex;
use x86_64::structures::paging::OffsetPageTable;

use super::{phys_allocator::PhysicalMemoryAllocator, MMapEntry};

pub static MEMORY_MANAGER: Mutex<OnceCell<MemoryManager<'static>>> = Mutex::new(OnceCell::new());

pub struct MemoryManager<'a> {
    mmap: Vec<MMapEntry>,
    phys_allocator: PhysicalMemoryAllocator,
    ident_map_pt: OffsetPageTable<'a>
}

impl<'a> MemoryManager<'a> {
    pub fn new(
        mmap: Vec<MMapEntry>,
        phys_allocator: PhysicalMemoryAllocator,
        identity_map: OffsetPageTable<'a>
    ) -> Self {
        Self {
            mmap,
            phys_allocator,
            ident_map_pt: identity_map
        }
    }

    // allocations
    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let mut heap = KERNEL_HEAP.lock();
        while heap.stats_alloc_actual() + layout.size() > heap.stats_total_bytes() {
            let frame = self.phys_allocator.alloc(1);
            heap.add_to_heap(frame.start, frame.end);
        }
        heap.alloc(layout).unwrap().as_ptr()
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        KERNEL_HEAP.lock().dealloc(NonNull::new_unchecked(ptr), layout)
    }

    fn alloc_kernel_page(&mut self) {
        let frame = self.phys_allocator.alloc(1);
    }

    // statistics
    pub fn heap_available() -> usize {
        KERNEL_HEAP.lock().stats_total_bytes()
    }

    pub fn heap_allocated() -> usize {
        KERNEL_HEAP.lock().stats_alloc_actual()
    }

    pub fn phys_memory_available(&self) -> usize {
        self.phys_allocator.available_bytes()
    }

    pub fn phys_memory_allocated(&self) -> usize {
        self.phys_allocator.allocated_bytes()
    }

    pub fn print_mmap_entries(&self) {
        for entry in &self.mmap {
            log::debug!("{:x?}", entry);
        }
    }
}

pub static KERNEL_HEAP: LockedHeap<36> = LockedHeap::empty();

pub struct KernelHeapDelegate;

unsafe impl GlobalAlloc for KernelHeapDelegate {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        MEMORY_MANAGER.force_unlock(); // DANGEROUS, for preventing deadlocks
        if let Some(manager) = MEMORY_MANAGER.lock().get_mut() {
            manager.alloc(layout)
        }
        else {
            KERNEL_HEAP.alloc(layout)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        MEMORY_MANAGER.force_unlock(); // DANGEROUS, for preventing deadlocks
        if let Some(manager) = MEMORY_MANAGER.lock().get_mut() {
            manager.dealloc(ptr, layout)
        }
        else {
            KERNEL_HEAP.dealloc(ptr, layout)
        }
    }
}

impl KernelHeapDelegate {
    pub unsafe fn add_to_heap(&self, start: usize, end: usize) {
        KERNEL_HEAP.lock().add_to_heap(start, end);
    }
}

#[global_allocator]
pub static KERNEL_HEAP_DELEGATE: KernelHeapDelegate = KernelHeapDelegate;