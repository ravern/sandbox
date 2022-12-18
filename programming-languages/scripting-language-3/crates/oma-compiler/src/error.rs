use std::{fmt, io, rc::Rc};

use crate::{lex::Token, source::Span};

#[derive(Clone, Debug)]
pub enum CompileError {
  Parse(ParseError),
  Verify(VerifyError),
  Io(Rc<io::Error>),
}

impl fmt::Display for CompileError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Parse(error) => write!(f, "{}", error),
      Self::Verify(error) => write!(f, "{}", error),
      Self::Io(error) => write!(f, "{}", error),
    }
  }
}

#[derive(Clone, Debug)]
pub struct ParseError {
  span: Span,
  unexpected: ParseExpectation,
  expected: Option<ParseExpectation>,
}

impl ParseError {
  pub fn end<const N: usize>(
    span: Span,
    expected: Option<[u8; N]>,
  ) -> ParseError {
    ParseError {
      span,
      unexpected: ParseExpectation::Token(Token::End),
      expected: expected
        .map(|expected| ParseExpectation::Chars(expected.to_vec())),
    }
  }

  pub fn chars<const N: usize>(
    span: Span,
    unexpected: u8,
    expected: Option<[u8; N]>,
  ) -> ParseError {
    ParseError {
      span,
      unexpected: ParseExpectation::Char(unexpected),
      expected: expected
        .map(|expected| ParseExpectation::Chars(expected.to_vec())),
    }
  }

  pub fn tokens<const N: usize>(
    span: Span,
    unexpected: Token,
    expected: Option<[Token; N]>,
  ) -> ParseError {
    ParseError {
      span,
      unexpected: ParseExpectation::Token(unexpected),
      expected: expected
        .map(|expected| ParseExpectation::Tokens(expected.to_vec())),
    }
  }
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if let Some(expected) = &self.expected {
      writeln!(f, "unexpected {}, expected {}", self.unexpected, expected)?;
    } else {
      writeln!(f, "unexpected {}", self.unexpected)?;
    }

    write!(
      f,
      "--> {}:{}:{}
     |
 {:>3} | {}
     | {:>padding$}
 ",
      self.span.path().display(),
      self.span.line(),
      self.span.column(),
      self.span.line(),
      self.span.line_content(),
      "^".repeat(self.span.content().len()),
      padding = self.span.column() + self.span.content().len(),
    )
  }
}

#[derive(Clone, Debug)]
pub enum ParseExpectation {
  Tokens(Vec<Token>),
  Token(Token),
  Chars(Vec<u8>),
  Char(u8),
}

impl fmt::Display for ParseExpectation {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Tokens(tokens) => write_slice(f, tokens),
      Self::Token(token) => write!(f, "{}", token),
      Self::Chars(bytes) => write_slice(
        f,
        &bytes
          .into_iter()
          .map(|byte| *byte as char)
          .collect::<Vec<char>>(),
      ),
      Self::Char(byte) => write!(f, "{}", *byte as char),
    }
  }
}

#[derive(Clone, Debug)]
pub struct VerifyError {
  span: Span,
  reason: VerifyErrorReason,
}

impl VerifyError {
  pub fn invalid_case_pat(span: Span) -> Self {
    Self {
      span,
      reason: VerifyErrorReason::InvalidCasePat,
    }
  }

  pub fn invalid_assignee(span: Span) -> Self {
    Self {
      span,
      reason: VerifyErrorReason::InvalidAssignee,
    }
  }

  pub fn multiple_tag_arguments(span: Span) -> Self {
    Self {
      span,
      reason: VerifyErrorReason::MultipleTagArguments,
    }
  }

  pub fn unresolved_identifier(span: Span) -> Self {
    Self {
      span,
      reason: VerifyErrorReason::UnresolvedIdentifier,
    }
  }

  pub fn import_argument_not_string(span: Span) -> Self {
    Self {
      span,
      reason: VerifyErrorReason::ImportArgumentNotString,
    }
  }
}

impl fmt::Display for VerifyError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}
    --> {}:{}:{}
     |
 {:>3} | {}
     | {:>padding$}
 ",
      self.reason,
      self.span.path().display(),
      self.span.line(),
      self.span.column(),
      self.span.line(),
      self.span.line_content(),
      "^".repeat(self.span.content().len()),
      padding = self.span.column() + self.span.content().len(),
    )
  }
}

#[derive(Clone, Debug)]
pub enum VerifyErrorReason {
  InvalidCasePat,
  InvalidAssignee,
  MultipleTagArguments,
  ImportArgumentNotString,
  UnresolvedIdentifier,
}

impl fmt::Display for VerifyErrorReason {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::InvalidCasePat => write!(f, "invalid pattern for case arm"),
      Self::InvalidAssignee => write!(f, "cannot assign to expression"),
      Self::MultipleTagArguments => {
        write!(f, "more than 1 argument passed to tag expression")
      }
      Self::UnresolvedIdentifier => {
        write!(f, "unresolved identifier")
      }
      Self::ImportArgumentNotString => {
        write!(f, "string must be passed to import")
      }
    }
  }
}

fn write_slice<T>(f: &mut fmt::Formatter, items: &[T]) -> fmt::Result
where
  T: fmt::Display,
{
  let len = items.len();
  items
    .iter()
    .enumerate()
    .map(|(index, byte)| {
      write!(f, "{}", byte)?;
      if index < len - 1 {
        write!(f, ", ")?;
      }
      Ok(())
    })
    .collect::<Result<_, fmt::Error>>()?;
  Ok(())
}
