pub mod legacy;

use crate::memory::{Addressable, MemoryAddr};
use std::cmp::{Ord, Ordering, PartialEq};
use std::collections::BTreeMap;
use std::io::Read;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum Error {
    ReadFailed,
    WriteFailed,
    MissingDevice,
}

type Result<T> = std::result::Result<T, Error>;

/// Describes the memory range for a device on a bus.
#[derive(Eq, Debug)]
pub struct Range(pub MemoryAddr, pub usize);

impl Range {
    pub fn contains_addr(&self, addr: &MemoryAddr) -> bool {
        &self.0 <= addr && &(self.0.add_offset(self.1)) > addr
    }
}

impl PartialEq for Range {
    fn eq(&self, other: &Range) -> bool {
        self.0 == other.0
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Range) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Range) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

/// A bus of similiarly related devices.
pub struct Bus {
    devices: BTreeMap<Range, Arc<Mutex<Addressable>>>,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            devices: BTreeMap::new(),
        }
    }

    /// Insert a device into the bus with the given range. Ranges must not
    /// overlap between devices.
    ///
    /// TODO: Actually check for overlapping ranges.
    pub fn insert(&mut self, range: Range, dev: Arc<Mutex<Addressable>>) -> Result<()> {
        self.devices.insert(range, dev);
        Ok(())
    }

    pub fn read(&self, addr: MemoryAddr, bs: &mut [u8]) -> Result<()> {
        let (offset, dev) = self.device_at_addr(&addr).ok_or(Error::MissingDevice)?;
        dev.lock()
            .expect("failed to acquire mutex")
            .read(bs, offset)
            .map_err(|_| Error::ReadFailed)?;
        Ok(())
    }

    pub fn write(&self, addr: MemoryAddr, bs: &[u8]) -> Result<()> {
        let (offset, dev) = self.device_at_addr(&addr).ok_or(Error::MissingDevice)?;
        dev.lock()
            .expect("failed to acquire mutex")
            .write(bs, offset)
            .map_err(|_| Error::WriteFailed)?;
        Ok(())
    }

    /// Find the device at the given address on the bus. If the device exists,
    /// the device and the offset from the start of the device will be returned.
    fn device_at_addr(&self, addr: &MemoryAddr) -> Option<(MemoryAddr, &Mutex<Addressable>)> {
        for (range, dev) in self.devices.iter() {
            if range.contains_addr(&addr) {
                let offset = MemoryAddr(addr.0 - (range.0).0);
                return Some((offset, dev));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::{Error as MemError, Result as MemResult};
    use std::io::{Read, Write};
}
