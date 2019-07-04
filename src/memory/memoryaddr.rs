use std::convert::From;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct MemoryAddr(pub usize);

impl MemoryAddr {
    pub fn add_offset(&self, offset: usize) -> MemoryAddr {
        MemoryAddr(self.0 + offset)
    }
}

impl From<u32> for MemoryAddr {
    fn from(n: u32) -> Self {
        MemoryAddr(n as usize)
    }
}
