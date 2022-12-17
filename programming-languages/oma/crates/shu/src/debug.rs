use std::{fmt, rc::Rc};

#[derive(Clone, Debug)]
pub struct Info {
  pub spans: Vec<Rc<Span>>,
}

impl Info {
  pub fn span(&self, offset: usize) -> Option<Span> {
    self.spans.get(offset).map(|span| (&**span).clone())
  }
}

#[derive(Clone, Debug)]
pub struct Span {
  pub line: usize,
  pub column: usize,
  pub path: Rc<String>,
}

impl fmt::Display for Span {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}:{}:{}", self.path, self.line, self.column)
  }
}
