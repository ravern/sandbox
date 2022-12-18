use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use num_traits::FromPrimitive;
use result::Result;
use std::fmt::{Display, Formatter};
use std::io::Cursor;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::{i16, i32, i8, u16, u32, u8};

const OPERATION_CODE_SIZE: u8 = 5;
const OPERATION_TYPE_SIZE: u8 = 8 - OPERATION_CODE_SIZE;

#[derive(Debug, Eq, PartialEq, Primitive, Clone, Copy)]
pub enum OperationCode {
  Nope = 0x0,
  Push = 0x1,
  Pop = 0x2,
  Add = 0x3,
  Sub = 0x4,
  Mul = 0x5,
  Div = 0x6,
  Mod = 0x7,
  Jump = 0x8,
  Dup = 0x9,
  Eq = 0x10,
  Neq = 0x11,
}

impl OperationCode {
  pub fn from_operation(operation: u8) -> Result<OperationCode> {
    let operation_code = operation >> OPERATION_TYPE_SIZE;
    match OperationCode::from_u8(operation_code) {
      Some(operation_code) => Ok(operation_code),
      None => Err(format_err!("Invalid operation code {:#X}", operation_code)),
    }
  }

  pub fn allowed_operation_type(self, operation_type: OperationType) -> bool {
    match self {
      OperationCode::Nope => operation_type.is_nope(),
      OperationCode::Push => !operation_type.is_nope(),
      OperationCode::Pop => !operation_type.is_nope(),
      OperationCode::Add => !operation_type.is_nope(),
      OperationCode::Sub => !operation_type.is_nope(),
      OperationCode::Mul => !operation_type.is_nope(),
      OperationCode::Div => !operation_type.is_nope(),
      OperationCode::Mod => match operation_type {
        OperationType::Nope => false,
        OperationType::F32 => false,
        _ => true,
      },
      OperationCode::Jump => !operation_type.is_nope(),
      OperationCode::Dup => !operation_type.is_nope(),
      OperationCode::Eq => !operation_type.is_nope(),
      OperationCode::Neq => !operation_type.is_nope(),
    }
  }
}

#[derive(Debug, Eq, PartialEq, Primitive, Clone, Copy)]
pub enum OperationType {
  Nope = 0x0,
  U8 = 0x1,
  U16 = 0x2,
  U32 = 0x3,
  I8 = 0x4,
  I16 = 0x5,
  I32 = 0x6,
  F32 = 0x7,
}

impl OperationType {
  pub fn from_operation(operation: u8) -> Result<OperationType> {
    let operation_type = (operation << OPERATION_CODE_SIZE) >> OPERATION_CODE_SIZE;
    match OperationType::from_u8(operation_type) {
      Some(operation_type) => Ok(operation_type),
      None => Err(format_err!("Invalid operation type {:#X}", operation_type)),
    }
  }

  pub fn len(self) -> usize {
    match self {
      OperationType::Nope => 0,
      OperationType::U8 => 1,
      OperationType::U16 => 2,
      OperationType::U32 => 4,
      OperationType::I8 => 1,
      OperationType::I16 => 2,
      OperationType::I32 => 4,
      OperationType::F32 => 4,
    }
  }

  fn is_nope(self) -> bool {
    self == OperationType::Nope
  }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Operand {
  U8(u8),
  U16(u16),
  U32(u32),
  I8(i8),
  I16(i16),
  I32(i32),
  F32(f32),
}

impl Operand {
  pub fn new(bytes: Vec<u8>, operation_type: OperationType) -> Operand {
    let mut cursor = Cursor::new(bytes);
    match operation_type {
      OperationType::Nope => panic!("Could not create nope operand"),
      OperationType::U8 => Operand::U8(cursor.read_u8().unwrap()),
      OperationType::U16 => Operand::U16(cursor.read_u16::<LittleEndian>().unwrap()),
      OperationType::U32 => Operand::U32(cursor.read_u32::<LittleEndian>().unwrap()),
      OperationType::I8 => Operand::I8(cursor.read_i8().unwrap()),
      OperationType::I16 => Operand::I16(cursor.read_i16::<LittleEndian>().unwrap()),
      OperationType::I32 => Operand::I32(cursor.read_i32::<LittleEndian>().unwrap()),
      OperationType::F32 => Operand::F32(cursor.read_f32::<LittleEndian>().unwrap()),
    }
  }

  pub fn bytes(self) -> Vec<u8> {
    let mut bytes = Vec::new();
    match self {
      Operand::U8(operand) => bytes.write_u8(operand).unwrap(),
      Operand::U16(operand) => bytes.write_u16::<LittleEndian>(operand).unwrap(),
      Operand::U32(operand) => bytes.write_u32::<LittleEndian>(operand).unwrap(),
      Operand::I8(operand) => bytes.write_i8(operand).unwrap(),
      Operand::I16(operand) => bytes.write_i16::<LittleEndian>(operand).unwrap(),
      Operand::I32(operand) => bytes.write_i32::<LittleEndian>(operand).unwrap(),
      Operand::F32(operand) => bytes.write_f32::<LittleEndian>(operand).unwrap(),
    }
    bytes
  }

  pub fn is_zero(self) -> bool {
    match self {
      Operand::U8(operand) => operand == 0,
      Operand::U16(operand) => operand == 0,
      Operand::U32(operand) => operand == 0,
      Operand::I8(operand) => operand == 0,
      Operand::I16(operand) => operand == 0,
      Operand::I32(operand) => operand == 0,
      Operand::F32(operand) => operand == 0f32,
    }
  }

  pub fn to_u32(self) -> u32 {
    match self {
      Operand::U32(operand) => operand,
      _ => panic!("Could not convert operand to u32"),
    }
  }
}

impl Display for Operand {
  fn fmt(&self, fmt: &mut Formatter) -> ::std::fmt::Result {
    match self {
      Operand::U8(operand) => write!(fmt, "{:#X}", operand),
      Operand::U16(operand) => write!(fmt, "{:#X}", operand),
      Operand::U32(operand) => write!(fmt, "{:#X}", operand),
      Operand::I8(operand) => write!(fmt, "{:#X}", operand),
      Operand::I16(operand) => write!(fmt, "{:#X}", operand),
      Operand::I32(operand) => write!(fmt, "{:#X}", operand),
      Operand::F32(operand) => write!(fmt, "{:.2}", operand),
    }
  }
}

impl Add for Operand {
  type Output = Operand;

  fn add(self, other: Operand) -> Operand {
    match (self, other) {
      (Operand::U8(left), Operand::U8(right)) => Operand::U8(u8::wrapping_add(left, right)),
      (Operand::U16(left), Operand::U16(right)) => Operand::U16(u16::wrapping_add(left, right)),
      (Operand::U32(left), Operand::U32(right)) => Operand::U32(u32::wrapping_add(left, right)),
      (Operand::I8(left), Operand::I8(right)) => Operand::I8(i8::wrapping_add(left, right)),
      (Operand::I16(left), Operand::I16(right)) => Operand::I16(i16::wrapping_add(left, right)),
      (Operand::I32(left), Operand::I32(right)) => Operand::I32(i32::wrapping_add(left, right)),
      (Operand::F32(left), Operand::F32(right)) => Operand::F32(left + right),
      _ => panic!("Failed to add due to invalid operands"),
    }
  }
}

impl Sub for Operand {
  type Output = Operand;

  fn sub(self, other: Operand) -> Operand {
    match (self, other) {
      (Operand::U8(left), Operand::U8(right)) => Operand::U8(u8::wrapping_sub(left, right)),
      (Operand::U16(left), Operand::U16(right)) => Operand::U16(u16::wrapping_sub(left, right)),
      (Operand::U32(left), Operand::U32(right)) => Operand::U32(u32::wrapping_sub(left, right)),
      (Operand::I8(left), Operand::I8(right)) => Operand::I8(i8::wrapping_sub(left, right)),
      (Operand::I16(left), Operand::I16(right)) => Operand::I16(i16::wrapping_sub(left, right)),
      (Operand::I32(left), Operand::I32(right)) => Operand::I32(i32::wrapping_sub(left, right)),
      (Operand::F32(left), Operand::F32(right)) => Operand::F32(left - right),
      _ => panic!("Failed to subtract due to invalid operands"),
    }
  }
}

impl Mul for Operand {
  type Output = Operand;

  fn mul(self, other: Operand) -> Operand {
    match (self, other) {
      (Operand::U8(left), Operand::U8(right)) => Operand::U8(u8::wrapping_mul(left, right)),
      (Operand::U16(left), Operand::U16(right)) => Operand::U16(u16::wrapping_mul(left, right)),
      (Operand::U32(left), Operand::U32(right)) => Operand::U32(u32::wrapping_mul(left, right)),
      (Operand::I8(left), Operand::I8(right)) => Operand::I8(i8::wrapping_mul(left, right)),
      (Operand::I16(left), Operand::I16(right)) => Operand::I16(i16::wrapping_mul(left, right)),
      (Operand::I32(left), Operand::I32(right)) => Operand::I32(i32::wrapping_mul(left, right)),
      (Operand::F32(left), Operand::F32(right)) => Operand::F32(left * right),
      _ => panic!("Failed to multiply due to invalid operands"),
    }
  }
}

impl Div for Operand {
  type Output = Operand;

  fn div(self, other: Operand) -> Operand {
    match (self, other) {
      (Operand::U8(left), Operand::U8(right)) => Operand::U8(u8::wrapping_div(left, right)),
      (Operand::U16(left), Operand::U16(right)) => Operand::U16(u16::wrapping_div(left, right)),
      (Operand::U32(left), Operand::U32(right)) => Operand::U32(u32::wrapping_div(left, right)),
      (Operand::I8(left), Operand::I8(right)) => Operand::I8(i8::wrapping_div(left, right)),
      (Operand::I16(left), Operand::I16(right)) => Operand::I16(i16::wrapping_div(left, right)),
      (Operand::I32(left), Operand::I32(right)) => Operand::I32(i32::wrapping_div(left, right)),
      (Operand::F32(left), Operand::F32(right)) => Operand::F32(left / right),
      _ => panic!("Failed to divide due to invalid operands"),
    }
  }
}

impl Rem for Operand {
  type Output = Operand;

  fn rem(self, other: Operand) -> Operand {
    match (self, other) {
      (Operand::U8(left), Operand::U8(right)) => Operand::U8(u8::wrapping_rem(left, right)),
      (Operand::U16(left), Operand::U16(right)) => Operand::U16(u16::wrapping_rem(left, right)),
      (Operand::U32(left), Operand::U32(right)) => Operand::U32(u32::wrapping_rem(left, right)),
      (Operand::I8(left), Operand::I8(right)) => Operand::I8(i8::wrapping_rem(left, right)),
      (Operand::I16(left), Operand::I16(right)) => Operand::I16(i16::wrapping_rem(left, right)),
      (Operand::I32(left), Operand::I32(right)) => Operand::I32(i32::wrapping_rem(left, right)),
      _ => panic!("Failed to mod due to invalid operands"),
    }
  }
}
