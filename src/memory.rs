use core::slice::from_raw_parts;

use lazy_static::lazy_static;
use buddy_system_allocator::LockedHeap;
use limine::{MemmapEntry, NonNullPtr, MemmapResponse, MemoryMapEntryType};
use spin::Mutex;
use x86_64::{structures::paging::{PhysFrame, Size4KiB, OffsetPageTable, PageTable}, PhysAddr, registers::control::Cr3, VirtAddr};

use crate::println;

#[global_allocator]
static KERNEL_HEAP: LockedHeap<36> = LockedHeap::empty();

static MEMORYMAP_REQUEST: limine::MemmapRequest = limine::MemmapRequest::new(0);
static HHDM_REQUEST: limine::HhdmRequest = limine::HhdmRequest::new(0);

lazy_static! {
    pub static ref MEMORY_MANAGER: Mutex<MemoryManager<'static>> = Mutex::new({
        let frame_allocator = FrameAllocator::new(&MEMORYMAP_REQUEST.get_response().get().unwrap());
    
        let hhdm_offset = HHDM_REQUEST.get_response().get().unwrap().offset;
        let pt_frame = Cr3::read().0;
        let pt_addr = pt_frame.start_address();
        
        let pt = unsafe { &mut *(pt_addr.as_u64() as *mut PageTable) };
        let opt = unsafe { OffsetPageTable::new(
            pt, VirtAddr::new(hhdm_offset)) };

        MemoryManager::<'static>::new(frame_allocator, opt)
    });
}

pub struct MemoryManager<'a> {
    frame_allocator: FrameAllocator,
    kernel_page_table: OffsetPageTable<'a>
}

impl<'a> MemoryManager<'a> {
    pub fn new(frame_allocator: FrameAllocator, page_table: OffsetPageTable<'a>) -> MemoryManager<'a> {
        MemoryManager {
            frame_allocator,
            kernel_page_table: page_table
        }
    }

    pub fn add_mem_to_heap(&mut self, frame_count: usize) {
        // n frames by 4KB = 1MB
        for _ in 0..frame_count {
            use x86_64::structures::paging::FrameAllocator;
            let frame = self.frame_allocator.allocate_frame();
            if let Some(frame) = frame {
                unsafe {
                    KERNEL_HEAP.lock().add_to_heap(
                        frame.start_address().as_u64() as usize,
                        (frame.start_address() + frame.size()).as_u64() as usize);
                }
            }
        }
    }

    pub fn print_heap_usage(&self) {
        let allocated = KERNEL_HEAP.lock().stats_total_bytes() / 1024;
        let actual_allocated = KERNEL_HEAP.lock().stats_alloc_actual() / 1024;
        println!("Heap allocated data: {}kB", allocated);
        println!("Heap actual data: {}kB", actual_allocated);
    }
}

pub struct FrameAllocator {
    memory_map: &'static [NonNullPtr<MemmapEntry>],
    next: usize
}

impl FrameAllocator {
    pub fn new(memory_map: &MemmapResponse) -> Self {
        FrameAllocator {
            memory_map: unsafe { from_raw_parts(memory_map.entries.as_ptr(), memory_map.entry_count as usize) },
            next: 0
        }
    }

    // pub const fn empty() -> Self {
    //     FrameAllocator {
    //         memory_map: &[],
    //         next: 0
    //     }
    // }

    fn get_usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let usable_frames = self.memory_map.iter().filter(|r| {
            r.typ == MemoryMapEntryType::Usable
        });
        let addr_ranges = usable_frames.map(|f| {
            f.base..f.base+f.len
        });
        let frame_addrs = addr_ranges.flat_map(|f| {
            f.step_by(4096)
        });
        frame_addrs.map(|a| {
            PhysFrame::containing_address(PhysAddr::new(a))
        })
    }
}

unsafe impl x86_64::structures::paging::FrameAllocator<Size4KiB> for FrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.get_usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}