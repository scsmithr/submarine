use env_logger;
use log::error;
use vm;

fn main() {
    env_logger::init();

    let k = vm::KvmContext::new().unwrap();
    let v = vm::Vm::newTest(&k).unwrap();
}
