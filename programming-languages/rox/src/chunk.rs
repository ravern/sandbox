use std::convert::TryFrom;

use crate::value::Value;

pub struct Chunk {
  pub constants: Vec<Value>,
  pub code: Vec<u8>,
  pub lines: Vec<usize>,
}

impl Chunk {
  pub fn new() -> Chunk {
    Chunk {
      constants: Vec::new(),
      code: Vec::new(),
      lines: Vec::new(),
    }
  }

  pub fn write(&mut self, code: u8, line: usize) {
    self.code.push(code);
    self.lines.push(line);
  }

  pub fn add_constant(&mut self, constant: Value) -> u8 {
    self.constants.push(constant);
    u8::try_from(self.constants.len() - 1).expect("too many constants")
  }
}
