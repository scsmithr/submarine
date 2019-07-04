extern crate kvm_bindings;
extern crate kvm_ioctls;
extern crate log;

use crate::device::{Bus, Device};
use crate::memory::{MemoryAddr, Memory, Region};
use crate::memory::memorymap::{MemoryMmap};
use log::debug;
use std::io;
use std::io::Write;
use std::slice;

#[derive(Debug)]
pub enum Error {
    Kvm(io::Error),
    VcpuFd(io::Error),
    VcpuFailedRun(io::Error),
    VcpuRegisters(io::Error),
    VcpuUnhandled,
    VcpuFailedIO,
}

type Result<T> = std::result::Result<T, Error>;

pub struct KvmContext {
    kvm: kvm_ioctls::Kvm,
}

impl KvmContext {
    pub fn new() -> Result<Self> {
        let kvm = kvm_ioctls::Kvm::new().map_err(Error::Kvm)?;
        Ok(KvmContext { kvm: kvm })
    }
}

pub struct Vm {
    fd: kvm_ioctls::VmFd,
}

impl Vm {
    pub fn new(kvm: &KvmContext) -> Result<Self> {
        let fd = kvm.kvm.create_vm().map_err(Error::Kvm)?;
        Ok(Vm { fd: fd })
    }

    pub fn init_memory(&mut self, mem: MemoryMmap, kvm: &KvmContext) -> Result<()> {
        let mem_region = kvm_bindings::kvm_userspace_memory_region {
            slot: 0,
            guest_phys_addr: 0 as u64,
            memory_size: mem.size() as u64,
            userspace_addr: mem.as_ptr() as u64,
            flags: kvm_bindings::KVM_MEM_LOG_DIRTY_PAGES,
        };
        self.fd
            .set_user_memory_region(mem_region)
            .map_err(Error::Kvm)?;
        Ok(())
    }

    pub fn new_test(kvm: &KvmContext) -> Result<Self> {
        let fd = kvm.kvm.create_vm().map_err(Error::Kvm)?;
        let mem_size = 0x4000;
        let guest_addr: u64 = 0x1000;
        let load_addr: *mut u8 = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                mem_size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_ANONYMOUS | libc::MAP_SHARED | libc::MAP_NORESERVE,
                -1,
                0,
            ) as *mut u8
        };

        let slot = 0;
        let mem_region = kvm_bindings::kvm_userspace_memory_region {
            slot,
            guest_phys_addr: guest_addr,
            memory_size: mem_size as u64,
            userspace_addr: load_addr as u64,
            flags: kvm_bindings::KVM_MEM_LOG_DIRTY_PAGES,
        };
        fd.set_user_memory_region(mem_region).map_err(Error::Kvm)?;

        let x86_code = [
            0xba, 0xf8, 0x03, /* mov $0x3f8, %dx */
            0x00, 0xd8, /* add %bl, %al */
            0x04, b'0', /* add $'0', %al */
            0xee, /* out %al, %dx */
            0xec, /* in %dx, %al */
            0xc6, 0x06, 0x00, 0x80, 0x00, /* movl $0, (0x8000); This generates a MMIO Write.*/
            0x8a, 0x16, 0x00, 0x80, /* movl (0x8000), %dl; This generates a MMIO Read.*/
            0xf4, /* hlt */
        ];

        unsafe {
            let mut slice = slice::from_raw_parts_mut(load_addr, mem_size as usize);
            slice.write(&x86_code).unwrap();
        }

        let vcpu_fd = fd.create_vcpu(0).unwrap();

        let mut vcpu_sregs = vcpu_fd.get_sregs().unwrap();
        vcpu_sregs.cs.base = 0;
        vcpu_sregs.cs.selector = 0;
        vcpu_fd.set_sregs(&vcpu_sregs).unwrap();

        let mut vcpu_regs = vcpu_fd.get_regs().unwrap();
        vcpu_regs.rip = guest_addr;
        vcpu_regs.rax = 2;
        vcpu_regs.rbx = 3;
        vcpu_regs.rflags = 2;
        vcpu_fd.set_regs(&vcpu_regs).unwrap();

        loop {
            match vcpu_fd.run().expect("run failed") {
                kvm_ioctls::VcpuExit::IoIn(addr, data) => {
                    println!(
                        "Received io in exit, addr {:#x}, data: {:#x}",
                        addr, data[0]
                    );
                }
                kvm_ioctls::VcpuExit::IoOut(addr, data) => {
                    println!(
                        "Received io in exit, addr {:#x}, data: {:#x}",
                        addr, data[0]
                    );
                }
                kvm_ioctls::VcpuExit::MmioRead(addr, data) => {
                    println!("Received mmio read, addr {:#x}, data: {:#x}", addr, data[0]);
                }
                kvm_ioctls::VcpuExit::MmioWrite(addr, data) => {
                    println!(
                        "Received mmio write, addr {:#x}, data: {:#x}",
                        addr, data[0]
                    );
                }
                kvm_ioctls::VcpuExit::Hlt => {
                    let dirty_pages_bitmap = fd.get_dirty_log(slot, mem_size).unwrap();
                    let dirty_pages = dirty_pages_bitmap
                        .into_iter()
                        .map(|page| page.count_ones())
                        .fold(0, |dirty_page_count, i| dirty_page_count + i);
                    assert_eq!(dirty_pages, 1);
                    break;
                }
                r => panic!("Unexpected exit reason {:?}", r),
            }
        }

        Ok(Vm { fd })
    }
}

/// A wrapper around a KVM provided virtual cpu.
pub struct Vcpu {
    fd: kvm_ioctls::VcpuFd,
    mmio_bus: Bus,
    pio_bus: Bus,
}

impl Vcpu {
    /// Create a new virtual cpu with the provided io busses for the given vm.
    pub fn new(vm: &Vm, mmio_bus: Bus, pio_bus: Bus) -> Result<Self> {
        let vcpu_fd = vm.fd.create_vcpu(0).map_err(Error::VcpuFd)?;
        Ok(Vcpu {
            fd: vcpu_fd,
            mmio_bus,
            pio_bus,
        })
    }

    /// Sets the appropriate vcpu regs for booting into a linux kernel.
    pub fn configure_kernel_load(&self, vm: &Vm, kernel_start: MemoryAddr) -> Result<()> {
        let regs = kvm_bindings::kvm_regs{
            rip: kernel_start.0 as u64,
            ..Default::default()
        };
        self.fd.set_regs(&regs).map_err(Error::VcpuRegisters)?;
        Ok(())
    }

    /// Run until vcpu exit, handling io exits.
    ///
    /// Any other vcpu exit is treated as unhandled.
    pub fn run(&self) -> Result<()> {
        match self.fd.run().map_err(Error::VcpuFailedRun)? {
            kvm_ioctls::VcpuExit::IoIn(addr, data) => {
                self.pio_bus
                    .read(MemoryAddr(addr as usize), data)
                    .map_err(|_| Error::VcpuFailedIO)?;
                Ok(())
            }
            kvm_ioctls::VcpuExit::IoOut(addr, data) => {
                self.pio_bus
                    .write(MemoryAddr(addr as usize), data)
                    .map_err(|_| Error::VcpuFailedIO)?;
                Ok(())
            }
            kvm_ioctls::VcpuExit::MmioRead(addr, data) => {
                self.mmio_bus
                    .read(MemoryAddr(addr as usize), data)
                    .map_err(|_| Error::VcpuFailedIO)?;
                Ok(())
            }
            kvm_ioctls::VcpuExit::MmioWrite(addr, data) => {
                self.pio_bus
                    .write(MemoryAddr(addr as usize), data)
                    .map_err(|_| Error::VcpuFailedIO)?;

                Ok(())
            }
            kvm_ioctls::VcpuExit::Hlt => Err(Error::VcpuUnhandled),
            _ => Err(Error::VcpuUnhandled),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Helper for creating a vm for tests.
    fn new_test_vm() -> Vm {
        let kvm = KvmContext::new().unwrap();
        let vm = Vm::new(&kvm).unwrap();
        vm
    }

    #[test]
    fn new_vcpu() {
        let vm = new_test_vm();
        let pio_bus = Bus::new();
        let mmio_bus = Bus::new();
        let vcpu = Vcpu::new(&vm, mmio_bus, pio_bus).unwrap();
    }
}
