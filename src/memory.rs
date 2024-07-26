use core::{slice::from_raw_parts, ops::Range};

use lazy_static::lazy_static;
use buddy_system_allocator::{LockedHeap, FrameAllocator};
use limine::{MemmapEntry, NonNullPtr, MemoryMapEntryType};
use spin::Mutex;
use x86_64::{structures::paging::{PhysFrame, OffsetPageTable, PageTable}, PhysAddr, registers::control::Cr3, VirtAddr};

use crate::println;

#[global_allocator]
static KERNEL_HEAP: LockedHeap<36> = LockedHeap::empty();

pub static MEMORYMAP_REQUEST: limine::MemmapRequest = limine::MemmapRequest::new(0);
static HHDM_REQUEST: limine::HhdmRequest = limine::HhdmRequest::new(0);

lazy_static! {
    pub static ref MEMORY_MANAGER: Mutex<MemoryManager<'static>> = Mutex::new({
        let mmap_response = MEMORYMAP_REQUEST.get_response().get().unwrap();
        let mmap = unsafe { from_raw_parts(mmap_response.entries.as_ptr(), mmap_response.entry_count as usize) };

        // // find few frames to allocate FrameAllocator's BTrees
        // const N_INITIAL_FRAMES: usize = 16;
        // let mut initial_frames = [(0, 0); N_INITIAL_FRAMES];
        // for frame_i in 0..N_INITIAL_FRAMES {
        //     let frame = get_usable_frames(mmap).nth(frame_i).unwrap();
        //     initial_frames[frame_i] = physframe_to_address_range(frame);
        //     unsafe {
        //         KERNEL_HEAP.lock().add_to_heap(initial_frames[frame_i].0, initial_frames[frame_i].1);
        //     }
        // }

        // // this call requires heap to be ready
        // let mut frame_allocator = FrameAllocator::<36>::new();
        // // mark previously allocated single frame as used
        // for frame_i in 0..N_INITIAL_FRAMES {
        //     frame_allocator.add_frame(initial_frames[frame_i].0, initial_frames[frame_i].1);
        // }
        // frame_allocator.alloc(N_INITIAL_FRAMES);

        // // add rest of usable frames provided by Limine
        // for frame_range in get_usable_frame_ranges(&mmap) {
        //     frame_allocator.add_frame(frame_range.start as usize, frame_range.end as usize);
        // }

        // // n frames by 4KB
        // let n_frames = 32;
        // for frame_i in 0..n_frames {
        //     let frame = frame_allocator.alloc(frame_i);
        //     if let Some(frame) = frame {
        //         unsafe {
        //             KERNEL_HEAP.lock().add_to_heap(frame, (frame + n_frames) - 1);
        //         }
        //     }
        // }

        // find few frames to allocate FrameAllocator's BTrees
        const N_INITIAL_FRAMES: usize = 16;
        let mut initial_frames = [(0, 0); N_INITIAL_FRAMES];
        for frame_i in 0..N_INITIAL_FRAMES {
            let frame = get_usable_frames(mmap).nth(frame_i).unwrap();
            initial_frames[frame_i] = physframe_to_address_range(frame);
            unsafe {
                KERNEL_HEAP.lock().add_to_heap(initial_frames[frame_i].0, initial_frames[frame_i].1);
            }
        }

        let mut frame_allocator = FrameAllocator::<36>::new();
        for frame in initial_frames {
            frame_allocator.add_frame(frame.0, frame.1);
        }
        frame_allocator.alloc(N_INITIAL_FRAMES);
        for frame in get_usable_frame_ranges(&mmap) {
            frame_allocator.add_frame(frame.start as usize, frame.end as usize);
        }
    
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
    frame_allocator: FrameAllocator<36>,
    kernel_page_table: OffsetPageTable<'a>
}

impl<'a> MemoryManager<'a> {
    pub fn new(frame_allocator: FrameAllocator<36>, page_table: OffsetPageTable<'a>) -> MemoryManager<'a> {
        MemoryManager {
            frame_allocator,
            kernel_page_table: page_table
        }
    }

    /// Actually just an empty function, use it to activate lazy loaded MemoryManager singleton
    pub fn init(&self) {}

    /// Prints heap usage details to kernel output
    pub fn print_heap_usage(&self) {
        let allocated = KERNEL_HEAP.lock().stats_total_bytes() / 1024;
        let actual_allocated = KERNEL_HEAP.lock().stats_alloc_actual() / 1024;
        println!("Heap allocated data: {}kB", allocated);
        println!("Heap actual data: {}kB", actual_allocated);
    }
}

/// Very basic frame allocator intended for initializing temporary heap space
// pub struct BumpStyleFrameAllocator {
//     memory_map: &'static [NonNullPtr<MemmapEntry>],
//     next_to_add: usize,
//     next_to_return: usize
// }

// impl BumpStyleFrameAllocator {
//     pub fn new(memory_map: &MemmapResponse) -> Self {
//         BumpStyleFrameAllocator {
//             memory_map: unsafe { from_raw_parts(memory_map.entries.as_ptr(), memory_map.entry_count as usize) },
//             next_to_add: 0,
//             next_to_return: 0
//         }
//     }

//     fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
//         let frame = get_usable_frames(&self.memory_map).nth(self.next_to_add);
//         self.next_to_add += 1;
//         frame
//     }

//     fn return_allocated_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
//         let frame = get_usable_frames(&self.memory_map).nth(self.next_to_return);
//         self.next_to_return += 1;
//         frame
//     }
// }

/// Gathers and returns available memory frames basing on Limine memory map response
fn get_usable_frames(mmap: &'static [NonNullPtr<MemmapEntry>]) -> impl Iterator<Item = PhysFrame> {
    let usable_frames = mmap.iter().filter(|r| {
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

/// Gathers and returns available memory frame ranges basing on Limine memory map response
fn get_usable_frame_ranges(mmap: &'static [NonNullPtr<MemmapEntry>]) -> impl Iterator<Item = Range<u64>> {
    let usable_frames = mmap.iter().filter(|r| {
        r.typ == MemoryMapEntryType::Usable
    });
    usable_frames.map(|f| {
        f.base..f.base+f.len
    })
}

fn physframe_to_address_range(frame: PhysFrame) -> (usize, usize) {
    (
        frame.start_address().as_u64() as usize,
        (frame.start_address().as_u64() + frame.size()) as usize
    )
}

// pub fn init_memory() {
//     let mmap_response = MEMORYMAP_REQUEST.get_response().get().unwrap();
//     let mmap = unsafe { from_raw_parts(mmap_response.entries.as_ptr(), mmap_response.entry_count as usize) };

//     // count available memory
//     let mut total_mem = 0;
//     for frame in mmap {
//         if frame.typ == MemoryMapEntryType::Usable {
//             total_mem += frame.len;
//         }
//     }

//     let n_frames = total_mem / 1024 / 1024;
//     for n_frame in get_usable_frames(mmap) {

//     }
//     KERNEL_HEAP.lock().add_to_heap(start, end)
// }