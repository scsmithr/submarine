use std::io;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct MemoryAddr(pub usize);

impl MemoryAddr {
    pub fn add_offset(&self, offset: usize) -> MemoryAddr {
        MemoryAddr(self.0 + offset)
    }
}
