use crate::pos::Pos;
use crate::token::{Token, TokenKind};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Chunk {
  pub path: String,
  pub constants: Vec<Value>,
  pub identifiers: HashMap<String, usize>,
  pub poses: Vec<Pos>,
  pub instructions: Vec<Instruction>,
}

impl Chunk {
  pub fn new<S>(path: S) -> Chunk
  where
    S: Into<String>,
  {
    Chunk {
      path: path.into(),
      constants: Vec::new(),
      identifiers: HashMap::new(),
      poses: Vec::new(),
      instructions: Vec::new(),
    }
  }

  pub fn push(&mut self, instruction: Instruction, pos: Pos) {
    self.poses.push(pos);
    self.instructions.push(instruction);
  }

  /// Pushes `constant` into the `constants` pool and pushes a new
  /// `Instruction::Push` into `instructions` containing the index.
  pub fn push_push(&mut self, constant: Value, pos: Pos) {
    let index = self.constants.len();
    self.constants.push(constant);
    self.poses.push(pos);
    self.instructions.push(Instruction::Push(index));
  }

  /// Pushes `identifier` into the `identifiers` table and pushes a new
  /// `Instruction::Load` into `instructions` containing the index.
  pub fn push_load(&mut self, identifier: String, pos: Pos) {
    let index = self.identifiers.len();
    self.identifiers.entry(identifier).or_insert(index);
    self.poses.push(pos);
    self.instructions.push(Instruction::Load(index));
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instruction {
  Store(usize),
  Load(usize),
  Push(usize),
  Pop,
  And,
  Or,
  Not,
  Add,
  Subtract,
  Multiply,
  Divide,
  Power,
  Negate,
  Assign,
}

impl Instruction {
  pub fn from_token(token: &Token) -> Option<Instruction> {
    match token.kind {
      TokenKind::And => Some(Instruction::And),
      TokenKind::Or => Some(Instruction::Or),
      TokenKind::Not => Some(Instruction::Not),
      TokenKind::Add => Some(Instruction::Add),
      TokenKind::Subtract => Some(Instruction::Subtract),
      TokenKind::Multiply => Some(Instruction::Multiply),
      TokenKind::Divide => Some(Instruction::Divide),
      TokenKind::Power => Some(Instruction::Power),
      _ => None,
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Value {
  Number(f32),
}

impl Value {
  pub fn from_token(token: &Token) -> Option<Value> {
    match &token.kind {
      TokenKind::Number(number) => Some(Value::Number(*number)),
      _ => None,
    }
  }
}
