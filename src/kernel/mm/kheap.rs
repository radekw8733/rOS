use core::{alloc::{GlobalAlloc, Layout}, ptr::NonNull};

use buddy_system_allocator::LockedHeap;

use super::phys_allocator::PHYS_ALLOCATOR;

static _KHEAP: LockedHeap<36> = LockedHeap::empty();

pub struct KernelHeap;

unsafe impl GlobalAlloc for KernelHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        interrupts::without(|| {
            let mut heap = _KHEAP.lock();
            while heap.stats_alloc_actual() + layout.size() > heap.stats_total_bytes() {
                let frame = PHYS_ALLOCATOR.lock().get_mut().expect("PHYS_ALLOCATOR not initialized").alloc(1);
                heap.add_to_heap(frame.start, frame.end);
            }
            heap.alloc(layout).unwrap().as_ptr()
        })
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        interrupts::without(|| {
            _KHEAP.lock().dealloc(NonNull::new_unchecked(ptr), layout)
        })
    }
}

impl KernelHeap {
    pub unsafe fn add_to_heap(&self, start: usize, end: usize) {
        interrupts::without(|| {
            _KHEAP.lock().add_to_heap(start, end);
        });
    }

    // statistics
    pub fn heap_available() -> usize {
        interrupts::without(|| {
            _KHEAP.lock().stats_total_bytes()
        })
    }

    pub fn heap_allocated() -> usize {
        interrupts::without(|| {
            _KHEAP.lock().stats_alloc_actual()
        })
    }
}

#[global_allocator]
pub static KERNEL_HEAP: KernelHeap = KernelHeap;