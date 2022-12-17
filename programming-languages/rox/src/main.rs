use std::{env, fs, process};

use rox::*;

fn main() {
  let args: Vec<String> = env::args().collect();
  match args.len() {
    1 => repl(),
    2 => run_file(&args[1]),
    _ => {
      eprintln!("Usage: rox [path]");
      process::exit(64);
    }
  }
}

fn repl() {
  let mut vm = Vm::new();

  let mut rl = rustyline::Editor::<()>::new();
  let error = loop {
    let line = match rl.readline("> ") {
      Ok(line) => line,
      Err(error) => break error,
    };
    vm.interpret(&line);
  };
}

fn run_file(path: &str) {
  let source = match fs::read_to_string(path) {
    Ok(source) => source,
    Err(error) => {
      eprintln!("Error: {}", error);
      process::exit(74);
    }
  };
}
