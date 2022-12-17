use crate::chunk::{Chunk, Constant, Function};

const SECTION_INFO: u8 = 1;
const SECTION_DATA: u8 = 2;
const SECTION_CODE: u8 = 3;

const CONSTANT_STR: u8 = 1;
const CONSTANT_FUN: u8 = 2;

pub fn emit(function: Function) -> Vec<u8> {
  let mut bytes = Vec::new();

  bytes.extend([b'O', b'M', b'A', 1]);
  bytes.extend(emit_function(function));

  bytes
}

fn emit_chunk(chunk: Chunk) -> Vec<u8> {
  let mut bytes = Vec::new();

  bytes.extend(emit_data(chunk.data));

  bytes.push(SECTION_CODE);
  bytes.extend((chunk.code.len() as u64).to_le_bytes());
  bytes.extend(chunk.code);

  bytes
}

fn emit_data(data: Vec<Constant>) -> Vec<u8> {
  let mut bytes = Vec::new();

  bytes.push(SECTION_DATA);
  bytes.extend((data.len() as u64).to_le_bytes());

  for constant in data {
    bytes.extend(emit_constant(constant));
  }

  bytes
}

fn emit_constant(constant: Constant) -> Vec<u8> {
  let mut bytes = Vec::new();

  match constant {
    Constant::String(string) => {
      bytes.push(CONSTANT_STR);
      bytes.extend((string.bytes().len() as u64).to_le_bytes());
      bytes.extend(string.bytes());
    }
    Constant::Function(function) => {
      bytes.push(CONSTANT_FUN);
      bytes.extend(emit_function(function));
    }
  }

  bytes
}

fn emit_function(function: Function) -> Vec<u8> {
  let mut bytes = Vec::new();

  bytes.extend((function.arity as u64).to_le_bytes());
  bytes.extend(emit_chunk(function.chunk));
  bytes.extend((function.locals as u64).to_le_bytes());
  bytes.extend((function.upvalues.len() as u64).to_le_bytes());
  for (index, is_local) in function.upvalues {
    bytes.extend((index as u64).to_le_bytes());
    if is_local {
      bytes.push(1);
    } else {
      bytes.push(0);
    }
  }

  bytes
}
