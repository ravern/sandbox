use span::Span;

#[derive(Debug)]
pub struct Token {
  pub span: Span,
  pub kind: TokenKind,
}

#[derive(Debug)]
pub enum TokenKind {
  And,
  Or,
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
}
