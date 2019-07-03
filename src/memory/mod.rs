mod memoryaddr;
mod memorymap;

pub use memorymap::Error;
pub use memorymap::MemoryMap;

pub use memoryaddr::MemoryAddr;

pub struct MemoryRegion {
    start: MemoryAddr,
    mmap: MemoryMap,
}

impl MemoryRegion {
    pub fn get_start(&self) -> usize {
        self.start.0
    }

    pub fn size(&self) -> usize {
        self.mmap.size()
    }

    pub fn host_addr(&self) -> *mut u8 {
        self.mmap.as_ptr()
    }
}
