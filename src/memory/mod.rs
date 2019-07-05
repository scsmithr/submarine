mod memoryaddr;
pub use memoryaddr::MemoryAddr;

pub mod memorymap;

use std::io;
use std::io::{Read, Write};

#[derive(Debug)]
pub enum Error {
    OutOfBounds,
    ReadFailed(io::Error),
    WriteFailed(io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Memory: Region {}

pub trait Region: Addressable {
    /// Region length in bytes.
    fn len(&self) -> usize;

    /// Read from the reader into memory starting at address. The amount read
    /// will be returned
    fn read_from<F: Read>(&mut self, addr: MemoryAddr, f: &mut F, count: usize) -> Result<usize>
    where
        Self: Sized;

    /// Write to the writer starting at address. The amount written will be
    /// returned.
    fn write_to<F: Write>(&self, addr: MemoryAddr, f: &mut F, count: usize) -> Result<usize>
    where
        Self: Sized;
}

pub trait Addressable {
    /// Read from memory into the provided buffer start at address. The amount
    /// read will be returned.
    fn read(&self, buf: &mut [u8], addr: MemoryAddr) -> Result<usize>;

    /// Write to memory using the buffer starting at address. The amount written
    /// will be returned.
    fn write(&mut self, buf: &[u8], addr: MemoryAddr) -> Result<usize>;
}
