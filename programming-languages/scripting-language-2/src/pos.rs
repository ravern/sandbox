use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Pos {
  pub line: usize,
  pub column: usize,
}

impl Pos {
  pub fn new() -> Pos {
    Pos { line: 1, column: 1 }
  }
}

impl Display for Pos {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
    write!(fmt, "{}:{}", self.line, self.column)
  }
}
