mod device;
mod memory;
mod vm;
mod loader;

use std::io::{Cursor};
use env_logger;
use log::error;

fn main() {
    env_logger::init();

    let k = vm::KvmContext::new().unwrap();
    let mut v = vm::Vm::new(&k).unwrap();

    let mut mem = memory::memorymap::MemoryMmap::new(1 << 30).unwrap();

    let mut img = Vec::new();
    img.extend_from_slice(include_bytes!("/boot/vmlinuz-linux"));
    let info = loader::load_kernel(&mut mem, &mut Cursor::new(&img)).unwrap();

    v.init_memory(mem, &k).unwrap();

    let mmio_bus = device::Bus::new();
    let pio_bus = device::Bus::new();
    let vcpu = vm::Vcpu::new(&v, mmio_bus, pio_bus).unwrap();
    vcpu.configure_kernel_load(&v, info.entry_point).unwrap();
    loop {
        vcpu.run().unwrap();
    }
}
