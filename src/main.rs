mod device;
mod memory;
mod vm;
mod loader;

use env_logger;
use log::error;

fn main() {
    env_logger::init();

    let k = vm::KvmContext::new().unwrap();
    let v = vm::Vm::new_test(&k).unwrap();
}
