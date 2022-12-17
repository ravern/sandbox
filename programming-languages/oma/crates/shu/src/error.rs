use std::fmt;

use crate::{debug::Span, parse::ParseError, value::Value};

#[derive(Debug)]
pub struct Error {
  reason: Reason,
  trace: Trace,
}

impl Error {
  pub fn new(reason: Reason, trace: Trace) -> Error {
    Error { reason, trace }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "error: {}", self.reason)?;
    write!(f, "{}", self.trace)?;
    Ok(())
  }
}

#[derive(Debug)]
pub enum Reason {
  // Compiler errors
  Parse(ParseError),
  InvalidCode(usize),
  InvalidOpcode(u8),
  InvalidData(usize),
  InvalidUpvalue(usize),
  InvalidNativeLambda(usize),
  WrongConstantType,
  EmptyFrameStack,
  EmptyStack,
  // User errors
  Type,
  InvalidKey(Value),
  InvalidTag(Value),
  ValueNotCallable(Value),
  WrongArity,
}

impl fmt::Display for Reason {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      // Compiler errors
      Reason::Parse(_) => write!(f, "corrupt executable"),
      Reason::InvalidCode(offset) => write!(f, "tried to access invalid code: {}", offset),
      Reason::InvalidOpcode(code) => write!(f, "opcode is invalid: {}", code),
      Reason::InvalidData(constant) => write!(f, "tried to access invalid constant: {}", constant),
      Reason::InvalidUpvalue(index) => write!(f, "tried to access invalid upvalue: {}", index),
      Reason::InvalidNativeLambda(id) => write!(f, "tried to access invalid native lambda: {}", id),
      Reason::WrongConstantType => write!(f, "wrong constant type"),
      Reason::EmptyFrameStack => write!(f, "tried to pop empty frame stack"),
      Reason::EmptyStack => write!(f, "tried to pop empty stack"),
      // User errors
      Reason::Type => write!(f, "invalid type"),
      Reason::InvalidTag(value) => write!(f, "invalid tag: {:?}", value),
      Reason::InvalidKey(key) => write!(f, "invalid key: {:?}", key),
      Reason::ValueNotCallable(value) => write!(f, "value not callable: {:?}", value),
      Reason::WrongArity => write!(f, "wrong arity"),
    }
  }
}

#[derive(Debug)]
pub struct Trace {
  // List of frames when error occured.
  // From the least recent call to the most recent call.
  spans: Vec<Option<Span>>,
}

impl Trace {
  pub fn new() -> Trace {
    Trace { spans: Vec::new() }
  }

  pub fn push(&mut self, span: Option<Span>) {
    self.spans.push(span)
  }
}

impl fmt::Display for Trace {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for (index, span) in self.spans.iter().rev().enumerate() {
      let separator = if index == self.spans.len() - 1 {
        ""
      } else {
        "\n"
      };
      if let Some(span) = span {
        write!(f, "  in {}{}", span, separator)?;
      } else {
        write!(f, "  in <<unknown>>{}", separator)?;
      }
    }
    Ok(())
  }
}
