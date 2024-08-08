use core::cmp::Ordering;

#[cfg(feature = "limine")]
use limine::memory_map::EntryType;

pub mod kheap;
pub mod vm_manager;
pub mod phys_allocator;

#[derive(Clone, Copy)]
pub struct MemoryRegion {
    pub start: usize,
    pub end: usize
}

impl core::fmt::Debug for MemoryRegion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MemoryRegion")
            .field_with("start", |f| {
                f.write_fmt(format_args!("{:#x?}", self.start))
            })
            .field_with("end", |f| {
                f.write_fmt(format_args!("{:#x?}", self.end))
            });
        Ok(())
    }
}

impl PartialEq for MemoryRegion {
    fn eq(&self, other: &Self) -> bool {
        (other.start >= self.start && other.start <= self.end) ||
        (other.start <= self.start && self.end >= self.start)
    }
}
impl Eq for MemoryRegion {}

impl PartialOrd for MemoryRegion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            if self.eq(other) {
                Ordering::Equal
            }
            else if other.start >= self.start && other.start <= self.end {
                Ordering::Greater
            }
            else {
                Ordering::Less
            }
        )
    }
}

impl Ord for MemoryRegion {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MMapEntryType {
    Usable,
    Kernel,
    Reserved,
    Framebuffer,
}

#[cfg(feature = "limine")]
impl From<EntryType> for MMapEntryType {
    fn from(value: EntryType) -> Self {
        match value {
            EntryType::USABLE => Self::Usable,
            EntryType::KERNEL_AND_MODULES => Self::Kernel,
            EntryType::FRAMEBUFFER => Self::Framebuffer,
            _ => Self::Reserved
        }
    }
}

#[derive(Clone, Debug)]
pub struct MMapEntry {
    pub typ: MMapEntryType,
    pub range: MemoryRegion
}

impl PartialEq for MMapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.range.eq(&other.range)
    }
}

impl Eq for MMapEntry {}

impl PartialOrd for MMapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.range.partial_cmp(&other.range)
    }
}

impl Ord for MMapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.range.cmp(&other.range)
    }
}