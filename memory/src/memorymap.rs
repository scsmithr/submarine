use std::io;
use std::io::Read;
use std::io::Write;

#[derive(Debug)]
pub enum Error {
    OutOfBounds,
    ReadFailed(io::Error),
    WriteFailed(io::Error),
}

type Result<T> = std::result::Result<T, Error>;

pub struct MemoryMap {
    addr: *mut u8,
    size: usize,
}

impl MemoryMap {
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

        Ok(MemoryMap {
            addr: load_addr,
            size: size,
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn read_at(&self, mut buf: &mut [u8], pos: usize) -> Result<usize> {
        if pos >= self.size {
            return Err(Error::OutOfBounds);
        }
        unsafe {
            let slice: &[u8] = &std::slice::from_raw_parts(self.addr, self.size)[pos..];
            Ok(buf.write(slice).map_err(Error::ReadFailed)?)
        }
    }

    pub fn write_at(&mut self, buf: &[u8], pos: usize) -> Result<usize> {
        if pos >= self.size {
            return Err(Error::OutOfBounds);
        }
        unsafe {
            let mut slice = &mut self.as_mut_slice()[pos..];
            Ok(slice.write(buf).map_err(Error::WriteFailed)?)
        }
    }

    pub fn read_from<F: Read>(&mut self, src: &mut F, offset: usize, count: usize) -> Result<()> {
        let end = offset + count - 1;
        if end >= self.size {
            return Err(Error::OutOfBounds);
        }
        unsafe {
            let slice = &mut self.as_mut_slice()[offset..end];
            Ok(src.read_exact(slice).map_err(Error::WriteFailed)?)
        }
    }

    unsafe fn as_mut_slice(&self) -> &mut [u8] {
        std::slice::from_raw_parts_mut(self.addr, self.size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let m = MemoryMap::new(256).unwrap();
        assert_eq!(256, m.size);
    }

    #[test]
    fn write_all() {
        let mut m = MemoryMap::new(4).unwrap();
        let to_write = &[4, 5, 4, 3];
        let n = m.write_at(to_write, 0).unwrap();
        assert_eq!(4, n);
        let buf = &mut [0; 4];
        let n = m.read_at(buf, 0).unwrap();
        assert_eq!(4, n);
        assert_eq!(to_write, buf);
    }
}
