use crate::pos::Pos;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Token {
  pub pos: Pos,
  pub kind: TokenKind,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
  And,
  Or,
  Not,
  Add,
  Subtract,
  Multiply,
  Divide,
  Power,
  Assign,
  LeftParen,
  RightParen,
  Identifier(String),
  Number(f32),
  EndOfFile,
}

impl Display for Token {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
    let token = match &self.kind {
      TokenKind::And => "and",
      TokenKind::Or => "or",
      TokenKind::Not => "not",
      TokenKind::Add => "+",
      TokenKind::Subtract => "-",
      TokenKind::Multiply => "*",
      TokenKind::Divide => "/",
      TokenKind::Power => "^",
      TokenKind::Assign => "=",
      TokenKind::LeftParen => "(",
      TokenKind::RightParen => ")",
      TokenKind::Identifier(identifier) => &identifier,
      TokenKind::Number(number) => return write!(fmt, "{}", number),
      TokenKind::EndOfFile => "end of file",
    };
    write!(fmt, "{}", token)
  }
}
