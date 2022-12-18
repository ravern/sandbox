use std::rc::Rc;

use gc::{unsafe_empty_trace, Finalize, Trace};
use num_traits::FromPrimitive;

use crate::{debug::Info, opcode::Opcode};

#[derive(Debug, Finalize)]
pub struct Chunk {
  pub info: Option<Info>,
  pub data: Box<[Constant]>,
  pub code: Box<[u8]>,
}

impl Chunk {
  pub fn info(&self) -> Option<&Info> {
    self.info.as_ref()
  }

  pub fn constant(&self, index: usize) -> Option<&Constant> {
    self.data.get(index)
  }

  pub fn op(&self, offset: usize) -> Option<Op> {
    let mut op = Op {
      code: read_u8(&self.code, offset)?,
      operand: 0,
    };
    op.operand = match op.size() {
      0 => 0,
      1 => read_u8(&self.code, offset + 1)? as u64,
      2 => read_u16(&self.code, offset + 1)? as u64,
      4 => read_u32(&self.code, offset + 1)? as u64,
      8 => read_u64(&self.code, offset + 1)?,
      _ => return None,
    };
    Some(op)
  }
}

unsafe impl Trace for Chunk {
  unsafe_empty_trace!();
}

#[derive(Debug, Finalize, Trace)]
pub enum Constant {
  String(String),
  Function(Function),
}

impl Constant {
  pub fn as_str(&self) -> Option<&str> {
    if let Constant::String(string) = self {
      Some(string.as_str())
    } else {
      None
    }
  }

  pub fn as_function(&self) -> Option<&Function> {
    if let Constant::Function(function) = self {
      Some(function)
    } else {
      None
    }
  }
}

#[derive(Clone, Debug, Finalize, Trace)]
pub struct Function {
  pub arity: u64,
  pub chunk: Rc<Chunk>,
  pub locals: u64,
  pub upvalues: Vec<(u64, bool)>,
}

const CODE_MASK: u8 = 0b11111100;
const SIZE_MASK: u8 = 0b00000011;

pub struct Op {
  code: u8,
  operand: u64,
}

impl Op {
  pub fn opcode(&self) -> Option<Opcode> {
    Opcode::from_u8(self.code())
  }

  pub fn operand(&self) -> u64 {
    self.operand
  }

  // Returns the raw (unvalidated) opcode of this op.
  pub fn code(&self) -> u8 {
    (self.code & CODE_MASK) >> 2
  }

  // Returns the byte size of the operand.
  pub fn size(&self) -> u8 {
    if let Some(opcode) = self.opcode() {
      if opcode.has_operand() {
        return 2u8.pow((self.code & SIZE_MASK) as u32);
      }
    }
    0
  }
}

fn read_u8(bytes: &[u8], offset: usize) -> Option<u8> {
  bytes.get(offset).copied()
}

fn read_u16(bytes: &[u8], offset: usize) -> Option<u16> {
  let mut read_bytes = [0u8; 2];
  read_bytes.clone_from_slice(bytes.get(offset..offset + 2)?);
  Some(u16::from_le_bytes(read_bytes))
}

fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
  let mut read_bytes = [0u8; 4];
  read_bytes.clone_from_slice(bytes.get(offset..offset + 4)?);
  Some(u32::from_le_bytes(read_bytes))
}

fn read_u64(bytes: &[u8], offset: usize) -> Option<u64> {
  let mut read_bytes = [0u8; 8];
  read_bytes.clone_from_slice(bytes.get(offset..offset + 8)?);
  Some(u64::from_le_bytes(read_bytes))
}
