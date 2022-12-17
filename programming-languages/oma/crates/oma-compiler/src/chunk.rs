use std::cmp::max;

use crate::debug::Info;

#[derive(Debug)]
pub struct Chunk {
  pub info: Option<Info>,
  pub data: Vec<Constant>,
  pub code: Vec<u8>,
}

impl Chunk {
  pub fn new() -> Chunk {
    Chunk {
      info: None,
      data: Vec::new(),
      code: Vec::new(),
    }
  }

  pub fn emit(&mut self, op: Op) -> usize {
    let offset = self.len();

    let (operand, size) = if let Some(operand) = op.operand {
      self.normalize_operand(operand)
    } else {
      (0, 0)
    };

    let opcode = (op.code as u8) << 2 | size;
    self.code.push(opcode);

    if op.code.has_operand() {
      match size {
        0 => self
          .code
          .extend(((operand & u8::MAX as u64) as u8).to_le_bytes()),
        1 => self
          .code
          .extend(((operand & u16::MAX as u64) as u16).to_le_bytes()),
        2 => self
          .code
          .extend(((operand & u32::MAX as u64) as u32).to_le_bytes()),
        3 => self
          .code
          .extend(((operand & u64::MAX as u64) as u64).to_le_bytes()),
        _ => unreachable!(),
      }
    }

    offset
  }

  // Panics if offset is invalid or if the sizes are mismatched.
  pub fn patch(&mut self, offset: usize, operand: Operand) {
    let (operand, _) = self.normalize_operand(operand);
    let size = self.code.get(offset).unwrap() & 3;

    match size {
      0 => self
        .code
        .get_mut(offset + 1..offset + 2)
        .unwrap()
        .clone_from_slice(&((operand & u8::MAX as u64) as u8).to_le_bytes()),
      1 => self
        .code
        .get_mut(offset + 1..offset + 3)
        .unwrap()
        .clone_from_slice(&((operand & u16::MAX as u64) as u16).to_le_bytes()),
      2 => self
        .code
        .get_mut(offset + 1..offset + 5)
        .unwrap()
        .clone_from_slice(&((operand & u32::MAX as u64) as u32).to_le_bytes()),
      3 => self
        .code
        .get_mut(offset + 1..offset + 9)
        .unwrap()
        .clone_from_slice(&((operand & u64::MAX as u64) as u64).to_le_bytes()),
      _ => unreachable!(),
    }
  }

  pub fn len(&self) -> usize {
    self.code.len()
  }

  // Returns the operand as a sequence of 8 bytes and the true size of the
  // operand in terms of bytes in bytes.
  fn normalize_operand(&mut self, operand: Operand) -> (u64, u8) {
    match operand {
      Operand::F64(float) => (float.to_bits(), 3),
      Operand::I64(int) => (int as u64, 3),
      Operand::Usize(uint) => match uint {
        uint if uint <= u8::MAX as usize => (uint as u64, 0),
        uint if uint <= u16::MAX as usize => (uint as u64, 1),
        uint if uint <= u32::MAX as usize => (uint as u64, 2),
        uint if uint <= u64::MAX as usize => (uint as u64, 3),
        _ => unreachable!(),
      },
      Operand::String(string) => {
        let constant = self.add_constant(Constant::String(string));
        self.normalize_operand(Operand::Usize(constant))
      }
      Operand::Function(function) => {
        let constant = self.add_constant(Constant::Function(function));
        self.normalize_operand(Operand::Usize(constant))
      }
    }
  }

  fn add_constant(&mut self, constant: Constant) -> usize {
    self.data.push(constant);
    self.data.len() - 1
  }
}

#[derive(Debug)]
pub enum Constant {
  String(String),
  Function(Function),
}

#[derive(Debug)]
pub struct Op {
  pub code: Opcode,
  pub operand: Option<Operand>,
}

impl Op {
  pub fn new(code: Opcode) -> Op {
    Op {
      code,
      operand: None,
    }
  }

  pub fn with_operand(code: Opcode, operand: Operand) -> Op {
    Op {
      code,
      operand: Some(operand),
    }
  }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Opcode {
  Nul = 0,
  Tru,
  Fls,
  Flt,
  Int,
  Str,
  Arr,
  Map,
  Lmd,
  Nal,
  Pop,
  // Locals and upvalues
  Lod,
  Sav,
  Lou,
  Sau,
  Clu,
  // Code navigation
  Jmp,
  Jit,
  Jif,
  // Stack manipulation
  Dup,
  Swp,
  Rot,
  // Primitive operations
  Add,
  Sub,
  Mul,
  Div,
  Rem,
  Neg,
  Eql,
  Neq,
  Gtn,
  Gte,
  Ltn,
  Lte,
  Not,
  // Map/array operations
  Get,
  Set,
  Apn,
  // Tagged operations
  Tag,
  Utg,
  Gtg,
  // Lambda operations
  Cal,
  Ret,
}

impl Opcode {
  pub fn has_operand(&self) -> bool {
    if let Opcode::Flt
    | Opcode::Int
    | Opcode::Str
    | Opcode::Lmd
    | Opcode::Nal
    | Opcode::Lod
    | Opcode::Sav
    | Opcode::Lou
    | Opcode::Sau
    | Opcode::Jmp
    | Opcode::Jit
    | Opcode::Jif
    | Opcode::Cal = self
    {
      true
    } else {
      false
    }
  }
}

#[derive(Debug)]
pub enum Operand {
  F64(f64),
  I64(i64),
  Usize(usize),
  String(String),
  Function(Function),
}

#[derive(Debug)]
pub struct Function {
  pub arity: u64,
  pub chunk: Chunk,
  pub locals: u64,
  pub upvalues: Vec<(u64, bool)>,
}
