#[derive(Debug)]
pub struct Span {
  pub path: String,
  pub from: Position,
  pub to: Position,
}

#[derive(Debug)]
pub struct Position {
  pub line: usize,
  pub column: usize,
}
