use shu::Vm;

fn main() {
  let mut vm = Vm::new();
  if let Err(error) = vm.run() {
    eprintln!("{}", error);
  }
}
