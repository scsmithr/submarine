extern crate vm;

fn main() {
    let k = vm::KvmContext::new().unwrap();
    let v = vm::Vm::new(&k).unwrap();
}
