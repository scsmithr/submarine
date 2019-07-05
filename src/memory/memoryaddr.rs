use std::convert::From;
use std::fmt;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct MemoryAddr(pub usize);

impl fmt::Display for MemoryAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

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
