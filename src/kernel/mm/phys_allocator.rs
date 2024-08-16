use core::cell::OnceCell;

use spin::Mutex;
use x86_64::{structures::paging::{FrameAllocator, FrameDeallocator, PhysFrame, Size4KiB}, PhysAddr};

use super::MemoryRegion;

pub static PHYS_ALLOCATOR: Mutex<OnceCell<PhysicalMemoryAllocator<36>>> = Mutex::new(OnceCell::new());

pub struct PhysicalMemoryAllocator<const ORDER: usize> {
    pub allocator: buddy_system_allocator::FrameAllocator<ORDER>,
    available: usize,
    allocated: usize,
}

impl<const ORDER: usize> PhysicalMemoryAllocator<ORDER> {
    pub fn new(allocator: buddy_system_allocator::FrameAllocator<ORDER>) -> Self {
        Self {
            allocator,
            available: 0,
            allocated: 0
        }
    }

    pub fn available_bytes(&self) -> usize {
        self.available
    }

    pub fn allocated_bytes(&self) -> usize {
        self.allocated
    }

    pub fn alloc(&mut self, count: usize) -> MemoryRegion {
        let frame = self.allocator.alloc(count).unwrap();
        self.allocated += (frame + 4095 * count) - frame;
        MemoryRegion {
            start: frame,
            end: frame + 4095 * count
        }
    }

    pub fn dealloc(&mut self, region: MemoryRegion) {
        self.allocated -= region.end - region.start;
        self.allocator.dealloc(region.start, (region.end - region.start + 4095) / 4096)
    }

    pub fn add_frame(&mut self, region: MemoryRegion) {
        self.available += region.end - region.start;
        self.allocator.add_frame(region.start, region.end);
    }
}

unsafe impl<const ORDER: usize> FrameAllocator<Size4KiB> for PhysicalMemoryAllocator<ORDER> {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        Some(PhysFrame::containing_address(PhysAddr::new(self.alloc(1).start as u64)))
    }
}

impl<const ORDER: usize> FrameDeallocator<Size4KiB> for PhysicalMemoryAllocator<ORDER> {
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame<Size4KiB>) {
        self.dealloc(MemoryRegion {
            start: frame.start_address().as_u64() as usize,
            end: (frame.start_address() + frame.size()).as_u64() as usize
        });
    }
}