#![cfg_attr(rustfmt, rustfmt_skip)]

use shu::{NativeLambdaRegistry, Vm};
use oma::prelude::PRELUDE;
use oma_compiler::compile;

fn main() {
  let mut registry = Vec::new();
  for (index, (name, _)) in PRELUDE.iter().enumerate() {
    registry.push((*name, index));
  }

  let bytes = match compile(registry) {
    Ok(bytes) => bytes,
    Err(error) => {
      eprintln!("error: {}", error);
      std::process::exit(1);
    },
  };

  let mut registry = NativeLambdaRegistry::new();
  for (_, lambda) in PRELUDE {
    registry.add(lambda);
  }

  let mut vm = Vm::new();
  if let Err(error) = vm.run(registry, &bytes) {
    eprintln!("{}", error);
  }
}
