use crate::memory::{Addressable, MemoryAddr, Result};

pub struct Serial {}

impl Serial {
    pub fn new() -> Self {
        Serial {}
    }
}

impl Addressable for Serial {
    fn read(&self, buf: &mut [u8], addr: MemoryAddr) -> Result<usize> {
        println!("read: ({}) {:?}", addr, buf);
        Ok(0)
    }

    fn write(&mut self, buf: &[u8], addr: MemoryAddr) -> Result<usize> {
        println!("write: ({}) {:?}", addr, buf);
        Ok(0)
    }
}
