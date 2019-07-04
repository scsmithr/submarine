mod device;
mod memory;
mod vm;
mod loader;

use env_logger;
use log::error;

fn main() {
    env_logger::init();

    let k = vm::KvmContext::new().unwrap();
    let mut v = vm::Vm::new(&k).unwrap();

    let mem = memory::memorymap::MemoryMmap::new(1 << 30).unwrap();

    let mut img = Vec::new();
    img.extend_from_slice(include_bytes!("/boot/vmlinuz-linux"));

    v.init_memory(mem, &k).unwrap();
}
