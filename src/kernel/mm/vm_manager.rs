use alloc::vec::Vec;

use super::MMapEntry;

pub struct VirtualMemoryManager {
    regions: Vec<MMapEntry>
}

// TODO: implement loading executables into memory when ELF loader is finished