// See https://www.kernel.org/doc/Documentation/x86/boot.txt for boot docs.

extern crate memory;

use memory::MemoryMap;

pub enum Error {}

type Result<T> = std::result::Result<T, Error>;

pub fn configure_boot(mmap: &MemoryMap) -> Result<()> {
    Ok(())
}
