mod device;
mod loader;
mod memory;
mod vm;

use device::legacy::Serial;
use env_logger;
use log::debug;
use memory::{Addressable, Region};
use std::io::Cursor;
use std::sync::{Arc, Mutex};

fn main() {
    env_logger::init();

    let k = vm::KvmContext::new().unwrap();
    let mut v = vm::Vm::new(&k).unwrap();

    let mut mem = memory::memorymap::MemoryMmap::new(1 << 30).unwrap();

    let mut img = Vec::new();
    img.extend_from_slice(include_bytes!("/boot/vmlinuz-linux"));
    let info = loader::load_kernel(&mut mem, &mut Cursor::new(&img)).unwrap();

    v.init_memory(&mem, &k).unwrap();

    let mut vcpu = vm::Vcpu::new(&v).unwrap();
    vcpu.configure_kernel_load(&v, &mut mem, info.entry_point, info.heap_end)
        .unwrap();

    let mut mmio_bus = device::Bus::new();
    let dev = Arc::new(Mutex::new(mem));
    let len = dev.lock().unwrap().len();
    mmio_bus
        .insert(device::Range(memory::MemoryAddr(0), len), dev)
        .unwrap();
    let mut pio_bus = device::Bus::new();
    let serial = Arc::new(Mutex::new(Serial::new()));
    pio_bus
        .insert(device::Range(memory::MemoryAddr(0x3f8), 8), serial)
        .unwrap();
    vcpu.set_mmio_bus(mmio_bus);
    vcpu.set_pio_bus(pio_bus);

    loop {
        vcpu.run().unwrap();
        debug!("exited");
    }
}
