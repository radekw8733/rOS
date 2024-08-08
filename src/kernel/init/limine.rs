use alloc::{boxed::Box, vec::Vec};
use buddy_system_allocator::FrameAllocator;
use limine::{memory_map::{Entry, EntryType}, request::{FramebufferRequest, MemoryMapRequest}};
use x86_64::{registers::control::Cr3, structures::paging::{OffsetPageTable, PageTable}, VirtAddr};

use crate::{drivers::{tty::fb::FramebufferConsole, video::{framebuffer::GenericFramebuffer, Size}}, kernel::{log::{Console, LOGGER}, mm::{kheap::KERNEL_HEAP, phys_allocator::{PhysicalMemoryAllocator, PHYS_ALLOCATOR}, MMapEntry, MemoryRegion}}, println};

static FB_REQUEST: FramebufferRequest = FramebufferRequest::new();
static MEMORYMAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

pub fn add_framebuffer_console() {
    let limine_fb = FB_REQUEST.get_response().unwrap().framebuffers().next().unwrap();

    let fb = GenericFramebuffer::new(
        limine_fb.addr() as usize,
        Size::new(limine_fb.width() as u32, limine_fb.height() as u32)
    );

    let console = FramebufferConsole::new(fb);
    let console = Box::new(console) as Box<dyn Console + Send>;

    LOGGER.lock().add_console(console);
}

pub fn setup_memory() {
    let mmap = MEMORYMAP_REQUEST.get_response().unwrap().entries();

    // find some space for FrameAllocator which requires alloc

    // Limine maps first page with about 300kB in space, should be enough for early heap
    // TODO: first page needs 4kb frame alignment checking
    let mut frame_iterator = get_usable_frame_iterator(mmap);

    let first_usable_frame = frame_iterator.next().unwrap();
    unsafe { KERNEL_HEAP.add_to_heap(first_usable_frame.start, first_usable_frame.end) };

    let mut frame_allocator = PhysicalMemoryAllocator::new(FrameAllocator::new());
    frame_allocator.add_frame(first_usable_frame);
    frame_allocator.alloc(1);

    // add rest of frames to allocator
    for frame in frame_iterator {
        frame_allocator.add_frame(frame);
    }

    let pt_frame = Cr3::read().0;
    let pt_addr = pt_frame.start_address();
    
    let pt = unsafe { &mut *(pt_addr.as_u64() as *mut PageTable) };
    let ident_map = unsafe { OffsetPageTable::new(pt, VirtAddr::new(0)) };

    let mmap = mmap.iter()
        .map(|e| {
            MMapEntry {
                typ: e.entry_type.into(),
                range: MemoryRegion {
                    start: e.base as usize,
                    end: (e.base + e.length) as usize
                }
            }
        })
        .collect::<Vec<MMapEntry>>();

    for entry in mmap {
        println!("{:x?}", entry);
    }

    PHYS_ALLOCATOR.lock().set(frame_allocator).ok();
}

/// Gathers and returns available memory frames basing on Limine memory map response
fn get_usable_frame_iterator(mmap: &'static [&Entry]) -> impl Iterator<Item = MemoryRegion> {
    mmap.iter()
        .filter(|r| {
            r.entry_type == EntryType::USABLE
        })
        .map(|f| {
            f.base..f.base+f.length
        })
        .map(|a| {
            MemoryRegion { start: a.start as usize, end: a.end as usize}
        })
}