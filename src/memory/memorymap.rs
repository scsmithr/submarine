use std::io::{Read, Write};
use super::{Region, Memory, Error, Result};
use super::memoryaddr::MemoryAddr;

pub struct MemoryMmap {
    region: RegionMmap,
}

impl MemoryMmap {
    pub fn new(size: usize) -> Result<Self> {
        let region = RegionMmap::new(size)?;
        Ok(MemoryMmap{region})
    }

    pub fn as_ptr(&self) -> *mut u8 {
        self.region.as_ptr()
    }
}

impl Memory for MemoryMmap {}

impl Region for MemoryMmap {
    fn size(&self) -> usize {
        self.region.size()
    }

    fn read(&self, mut buf: &mut [u8], addr: MemoryAddr) -> Result<usize> {
        self.region.read(buf, addr)
    }

    fn write(&mut self, buf: &[u8], addr: MemoryAddr) -> Result<usize> {
        self.region.write(buf, addr)
    }

    fn read_from<F: Read>(&mut self, addr: MemoryAddr, f: &mut F, count: usize) -> Result<usize> {
        self.region.read_from(addr, f, count)
    }

    // TODO: Implement.
    fn write_to<F: Write>(&self, addr: MemoryAddr, f: &mut F, count: usize) -> Result<usize> {
        Ok(0)
    }
}

pub struct RegionMmap {
    addr: *mut u8,
    size: usize,
}

impl RegionMmap {
    pub fn new(size: usize) -> Result<Self> {
        let load_addr: *mut u8 = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_ANONYMOUS | libc::MAP_SHARED | libc::MAP_NORESERVE,
                -1,
                0,
            ) as *mut u8
        };

        Ok(RegionMmap {
            addr: load_addr,
            size: size,
        })
    }

    fn check_bounds(&self, addr: &MemoryAddr) -> Result<()> {
        if addr.0 >= self.size {
            return Err(Error::OutOfBounds);
        }
        Ok(())
    }

    pub fn as_ptr(&self) -> *mut u8 {
        self.addr
    }

    unsafe fn as_mut_slice(&self) -> &mut [u8] {
        std::slice::from_raw_parts_mut(self.addr, self.size)
    }
}

impl Region for RegionMmap {
    fn size(&self) -> usize {
        self.size
    }

    fn read(&self, mut buf: &mut [u8], addr: MemoryAddr) -> Result<usize> {
        self.check_bounds(&addr)?;
        unsafe {
            let slice: &[u8] = &std::slice::from_raw_parts(self.addr, self.size)[addr.0..];
            Ok(buf.write(slice).map_err(Error::ReadFailed)?)
        }
    }

    fn write(&mut self, buf: &[u8], addr: MemoryAddr) -> Result<usize> {
        self.check_bounds(&addr)?;
        unsafe {
            let mut slice = &mut self.as_mut_slice()[addr.0..];
            Ok(slice.write(buf).map_err(Error::WriteFailed)?)
        }
    }

    fn read_from<F: Read>(&mut self, addr: MemoryAddr, f: &mut F, count: usize) -> Result<usize> {
        let end = addr.0 + count - 1;
        if end >= self.size {
            return Err(Error::OutOfBounds);
        }
        unsafe {
            let slice = &mut self.as_mut_slice()[addr.0..end];
            Ok(f.read(slice).map_err(Error::ReadFailed)?)
        }
    }

    // TODO: Implement.
    fn write_to<F: Write>(&self, addr: MemoryAddr, f: &mut F, count: usize) -> Result<usize> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let m = RegionMmap::new(256).unwrap();
        assert_eq!(256, m.size);
    }

    #[test]
    fn write_all() {
        let mut m = RegionMmap::new(4).unwrap();
        let to_write = &[4, 5, 4, 3];
        let addr = MemoryAddr(0);
        let n = m.write(to_write, addr.clone()).unwrap();
        assert_eq!(4, n);
        let buf = &mut [0; 4];
        let n = m.read(buf, addr).unwrap();
        assert_eq!(4, n);
        assert_eq!(to_write, buf);
    }
}
