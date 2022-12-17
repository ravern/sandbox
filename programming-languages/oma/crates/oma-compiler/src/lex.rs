use std::fmt;

use crate::{
  error::{CompileError, ParseError},
  source::{Source, Span},
};

pub struct Lexer {
  source: Source,
  start: usize,
  end: usize,
}

impl Lexer {
  pub fn new(source: Source) -> Self {
    Self {
      source,
      start: 0,
      end: 0,
    }
  }

  pub fn next(&mut self) -> Result<Lexeme, CompileError> {
    self.whitespace();

    self.start = self.end;

    if self.start >= self.source.content().len() {
      return Ok(self.build(Token::End));
    }

    match self
      .peek()
      .expect("`self.start` checked to be within bounds")
    {
      b'"' => return self.string(),
      byte if is_digit(byte) => return self.number(),
      b'_' => return self.ident(),
      byte if is_alphabetic(byte) => return self.ident(),
      _ => {}
    }

    let lexeme = match (
      self
        .advance()
        .expect("`self.start` checked to be within bounds"),
      self.peek(),
    ) {
      (b'/', Some(b'/')) => self.comment()?,
      (b'+', _) => self.build(Token::Plus),
      (b'-', Some(b'>')) => self.advance_and_build(Token::Arrow),
      (b'-', _) => self.build(Token::Dash),
      (b'*', _) => self.build(Token::Star),
      (b'/', _) => self.build(Token::Slash),
      (b'_', _) => self.build(Token::Underscore),
      (b'.', Some(b'.')) => self.advance_and_build(Token::DotDot),
      (b'.', _) => self.build(Token::Dot),
      (b',', _) => self.build(Token::Comma),
      (b':', _) => self.build(Token::Colon),
      (b';', _) => self.build(Token::Semicolon),
      (b'(', _) => self.build(Token::OpenParen),
      (b')', _) => self.build(Token::CloseParen),
      (b'[', _) => self.build(Token::OpenBracket),
      (b']', _) => self.build(Token::CloseBracket),
      (b'{', _) => self.build(Token::OpenBrace),
      (b'}', _) => self.build(Token::CloseBrace),
      (b'&', Some(b'&')) => self.advance_and_build(Token::And),
      (b'|', Some(b'|')) => self.advance_and_build(Token::Or),
      (b'=', Some(b'=')) => self.advance_and_build(Token::EqualEqual),
      (b'=', _) => self.build(Token::Equal),
      (b'!', Some(b'=')) => self.advance_and_build(Token::BangEqual),
      (b'!', _) => self.build(Token::Bang),
      (b'>', Some(b'=')) => self.advance_and_build(Token::GreaterEqual),
      (b'>', _) => self.build(Token::Greater),
      (b'<', Some(b'=')) => self.advance_and_build(Token::LessEqual),
      (b'<', _) => self.build(Token::Less),
      (byte, _) => return Err(self.build_error::<0>(byte, None)),
    };

    Ok(lexeme)
  }

  fn comment(&mut self) -> Result<Lexeme, CompileError> {
    self
      .expect([b'/'])
      .expect("`self.comment` should begin with slash");

    loop {
      if let Some(b'\n') | None = self.peek() {
        break;
      }
      self.advance().expect("`self.peek` returned a byte");
    }

    Ok(self.build(Token::Comment))
  }

  fn string(&mut self) -> Result<Lexeme, CompileError> {
    self
      .expect([b'"'])
      .expect("`self.string` should begin with double quote");

    loop {
      match self.peek() {
        Some(b'"') => break,
        None => return Err(self.build_end_error(Some([b'"']))),
        _ => self.advance(),
      };
    }

    self
      .expect([b'"'])
      .expect("closing quote was peeked earlier");

    Ok(self.build(Token::String))
  }

  fn number(&mut self) -> Result<Lexeme, CompileError> {
    let mut has_decimal = false;

    loop {
      match self.peek() {
        Some(b'.') if !has_decimal => {
          has_decimal = true;
          self.advance();
        }
        Some(byte) if is_digit(byte) => {
          self.advance();
        }
        Some(_) => break,
        None => break,
      }
    }

    Ok(self.build(Token::Number))
  }

  fn ident(&mut self) -> Result<Lexeme, CompileError> {
    loop {
      match self.peek() {
        Some(byte)
          if !is_digit(byte) && !is_alphabetic(byte) && byte != b'_' =>
        {
          break
        }
        None => break,
        _ => self.advance(),
      };
    }

    let lexeme = match self.span().content() {
      "try" => self.build(Token::Try),
      "let" => self.build(Token::Let),
      "if" => self.build(Token::If),
      "else" => self.build(Token::Else),
      "case" => self.build(Token::Case),
      "for" => self.build(Token::For),
      "in" => self.build(Token::In),
      "while" => self.build(Token::While),
      "true" => self.build(Token::Bool),
      "false" => self.build(Token::Bool),
      "null" => self.build(Token::Null),
      _ => self.build(Token::Ident),
    };

    Ok(lexeme)
  }

  fn whitespace(&mut self) {
    loop {
      match self.peek() {
        Some(byte) if is_whitespace(byte) => {
          self.advance().expect("`self.peek` returned `Some`");
        }
        _ => break,
      }
    }
  }

  fn build(&self, token: Token) -> Lexeme {
    Lexeme {
      span: self.span(),
      token,
    }
  }

  fn advance_and_build(&mut self, token: Token) -> Lexeme {
    self.advance().unwrap();
    self.build(token)
  }

  fn build_error<const N: usize>(
    &self,
    unexpected: u8,
    expected: Option<[u8; N]>,
  ) -> CompileError {
    CompileError::Parse(ParseError::chars(self.span(), unexpected, expected))
  }

  fn build_end_error<const N: usize>(
    &self,
    expected: Option<[u8; N]>,
  ) -> CompileError {
    CompileError::Parse(ParseError::end(self.span(), expected))
  }

  fn span(&self) -> Span {
    Span::new(self.source.clone(), self.start, self.end)
  }

  fn peek(&self) -> Option<u8> {
    self.source.content().as_bytes().get(self.end).copied()
  }

  fn advance(&mut self) -> Option<u8> {
    if let Some(byte) = self.peek() {
      self.end += 1;
      Some(byte)
    } else {
      None
    }
  }

  fn expect<const N: usize>(
    &mut self,
    expected: [u8; N],
  ) -> Result<u8, CompileError> {
    let byte = self.advance().ok_or(self.build_end_error(Some(expected)))?;

    if expected.contains(&byte) {
      Ok(byte)
    } else {
      Err(self.build_error(byte, Some(expected)))
    }
  }
}

#[derive(Clone, Debug)]
pub struct Lexeme {
  span: Span,
  token: Token,
}

impl Lexeme {
  pub fn span(&self) -> &Span {
    &self.span
  }

  pub fn token(&self) -> Token {
    self.token
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
  // Literals
  Number,
  Bool,
  String,
  Ident,
  Null,
  // Punctuation
  Plus,
  Dash,
  Star,
  Slash,
  Dot,
  DotDot,
  Comma,
  Colon,
  Semicolon,
  Underscore,
  Arrow,
  Equal,
  EqualEqual,
  Bang,
  BangEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,
  And,
  Or,
  OpenParen,
  CloseParen,
  OpenBracket,
  CloseBracket,
  OpenBrace,
  CloseBrace,
  // Keywords
  Try,
  Let,
  Case,
  If,
  Else,
  For,
  In,
  While,
  // Misc.
  Comment,
  End,
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Number => write!(f, "number"),
      Self::Bool => write!(f, "bool"),
      Self::String => write!(f, "string"),
      Self::Ident => write!(f, "identifier"),
      Self::Null => write!(f, "null"),
      Self::Plus => write!(f, "'+'"),
      Self::Dash => write!(f, "'-'"),
      Self::Star => write!(f, "'*'"),
      Self::Slash => write!(f, "'/'"),
      Self::Dot => write!(f, "'.'"),
      Self::DotDot => write!(f, "'..'"),
      Self::Comma => write!(f, "','"),
      Self::Colon => write!(f, "':'"),
      Self::Semicolon => write!(f, "';'"),
      Self::Underscore => write!(f, "'_'"),
      Self::Arrow => write!(f, "'->'"),
      Self::Equal => write!(f, "'='"),
      Self::EqualEqual => write!(f, "'=='"),
      Self::Bang => write!(f, "'!'"),
      Self::BangEqual => write!(f, "'!='k"),
      Self::Greater => write!(f, "'>'"),
      Self::GreaterEqual => write!(f, "'>='"),
      Self::Less => write!(f, "'<'"),
      Self::LessEqual => write!(f, "'<='"),
      Self::And => write!(f, "'&&'"),
      Self::Or => write!(f, "'||'"),
      Self::OpenParen => write!(f, "'('"),
      Self::CloseParen => write!(f, "')'"),
      Self::OpenBracket => write!(f, "'['"),
      Self::CloseBracket => write!(f, "']'"),
      Self::OpenBrace => write!(f, "'{{'"),
      Self::CloseBrace => write!(f, "'}}'"),
      Self::Try => write!(f, "try"),
      Self::Let => write!(f, "let"),
      Self::Case => write!(f, "case"),
      Self::If => write!(f, "if"),
      Self::Else => write!(f, "else"),
      Self::For => write!(f, "for"),
      Self::In => write!(f, "in"),
      Self::While => write!(f, "while"),
      Self::Comment => write!(f, "comment"),
      Self::End => write!(f, "end of input"),
    }
  }
}

fn is_digit(byte: u8) -> bool {
  byte >= b'0' && byte <= b'9'
}

fn is_whitespace(byte: u8) -> bool {
  byte == b'\r' || byte == b' ' || byte == b'\t' || byte == b'\n'
}

fn is_alphabetic(byte: u8) -> bool {
  byte >= b'a' && byte <= b'z' || byte >= b'A' && byte <= b'Z'
}
