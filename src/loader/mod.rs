// See https://www.kernel.org/doc/Documentation/x86/boot.txt for boot docs.

extern crate log;
extern crate boot_gen;

use crate::memory::{MemoryAddr, Memory, Error as MemoryError};
use boot_gen::bootparam::setup_header;
use log::debug;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::mem;

#[derive(Debug)]
pub enum Error {
    KernelSeekEnd(io::Error),
    KernelSeekHdr(io::Error),
    KernelMemoryLoad(MemoryError),

    InvalidImage,

    ReadStruct(io::Error),
}

type Result<T> = std::result::Result<T, Error>;

const K_HDR_OFFSET: u16 = 0x01F1;
const K_HDR_MAGIC: u32 = 0x5372_6448;

const K_BZ_LOAD_ADDR: u32 = 0x0010_0000;

const K_64BIT_OFFSET: u16 = 0x0200;

pub struct LoadInfo {
    pub kernel_start: MemoryAddr,
    pub entry_point: MemoryAddr,
    pub heap_end: MemoryAddr,
}

/// Load the kernel image into memory.
pub fn load_kernel<F: Read + Seek, M: Memory>(mem: &mut M, image: &mut F) -> Result<LoadInfo> {
    let mut kernel_size = image.seek(SeekFrom::End(0)).map_err(Error::KernelSeekEnd)? as usize;
    let mut hdr = setup_header::default();
    image
        .seek(SeekFrom::Start(K_HDR_OFFSET as u64))
        .map_err(Error::KernelSeekHdr)?;

    unsafe {
        read_struct(image, &mut hdr)?;
    }

    if hdr.header != K_HDR_MAGIC {
        return Err(Error::InvalidImage);
    }

    let mut setup_size = hdr.setup_sects as usize;
    if hdr.setup_sects == 0 {
        setup_size = 4 // Backwards compat.
    }
    setup_size = (setup_size + 1) * 512;
    kernel_size -= setup_size;

    hdr.code32_start = K_BZ_LOAD_ADDR;

    debug!("start: {}, count: {}", hdr.code32_start, kernel_size);
    mem.read_from(MemoryAddr::from(hdr.code32_start), image, kernel_size)
        .map_err(Error::KernelMemoryLoad)?;

    let info = LoadInfo{
        kernel_start: MemoryAddr::from(hdr.code32_start),
        entry_point: MemoryAddr(hdr.code32_start as usize + K_64BIT_OFFSET as usize),
        heap_end: MemoryAddr(hdr.code32_start as usize + kernel_size as usize),
    };
    Ok(info)
}

unsafe fn read_struct<F: Read, T>(f: &mut F, s: &mut T) -> Result<()> {
    let slice: &mut [u8] =
        std::slice::from_raw_parts_mut(s as *mut T as *mut u8, mem::size_of::<T>());
    f.read_exact(slice).map_err(Error::ReadStruct)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;
    use crate::memory::memorymap::MemoryMmap;

    fn read_bzimage() -> Vec<u8> {
        let mut bs = Vec::new();
        bs.extend_from_slice(include_bytes!("/boot/vmlinuz-linux"));
        bs
    }

    fn new_memory_map() ->  MemoryMmap{
        const SIZE: usize = 10 << 20;
        MemoryMmap::new(SIZE).unwrap()
    }

    #[test]
    fn no_panic() {
        let mut mem = new_memory_map();
        let mut img = read_bzimage();
        load_kernel(&mut mem, &mut Cursor::new(&img)).unwrap();
    }
}
