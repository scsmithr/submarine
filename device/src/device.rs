extern crate memory;

use memory::MemoryAddr;
use std::io;

pub enum Error {
    ReadFailed,
    WriteFailed,
}

type Result<T> = std::result::Result<T, Error>;

pub trait Device {
    fn read(&mut self, addr: MemoryAddr, bs: &mut [u8]) -> Result<()>;
    fn write(&mut self, addr: MemoryAddr, bs: [u8]) -> Result<()>;
}
