use std::{fmt, rc::Rc};

#[derive(Debug)]
pub struct Token {
  pub ty: TokenType,
  pub line: usize,
  pub source: Rc<String>,
  pub start: usize,
  pub len: usize,
}

impl Token {
  pub fn as_str(&self) -> &str {
    &self.source[self.start..self.start + self.len]
  }
}

impl Clone for Token {
  fn clone(&self) -> Self {
    Self {
      ty: self.ty.clone(),
      line: self.line,
      source: Rc::clone(&self.source),
      start: self.start,
      len: self.len,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  Comma,
  Dot,
  Minus,
  Plus,
  Semicolon,
  Slash,
  Star,
  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,
  Identifier,
  String,
  Number,
  And,
  Class,
  Else,
  False,
  For,
  Fun,
  If,
  Nil,
  Or,
  Print,
  Return,
  Super,
  This,
  True,
  Var,
  While,
  Eof,
  Error,
}

impl fmt::Display for TokenType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      TokenType::LeftParen => write!(f, "LeftParen"),
      TokenType::RightParen => write!(f, "RightParen"),
      TokenType::LeftBrace => write!(f, "LeftBrace"),
      TokenType::RightBrace => write!(f, "RightBrace"),
      TokenType::Comma => write!(f, "Comma"),
      TokenType::Dot => write!(f, "Dot"),
      TokenType::Minus => write!(f, "Minus"),
      TokenType::Plus => write!(f, "Plus"),
      TokenType::Semicolon => write!(f, "Semicolon"),
      TokenType::Slash => write!(f, "Slash"),
      TokenType::Star => write!(f, "Star"),
      TokenType::Bang => write!(f, "Bang"),
      TokenType::BangEqual => write!(f, "BangEqual"),
      TokenType::Equal => write!(f, "Equal"),
      TokenType::EqualEqual => write!(f, "EqualEqual"),
      TokenType::Greater => write!(f, "Greater"),
      TokenType::GreaterEqual => write!(f, "GreaterEqual"),
      TokenType::Less => write!(f, "Less"),
      TokenType::LessEqual => write!(f, "LessEqual"),
      TokenType::Identifier => write!(f, "Identifier"),
      TokenType::String => write!(f, "String"),
      TokenType::Number => write!(f, "Number"),
      TokenType::And => write!(f, "And"),
      TokenType::Class => write!(f, "Class"),
      TokenType::Else => write!(f, "Else"),
      TokenType::False => write!(f, "False"),
      TokenType::For => write!(f, "For"),
      TokenType::Fun => write!(f, "Fun"),
      TokenType::If => write!(f, "If"),
      TokenType::Nil => write!(f, "Nil"),
      TokenType::Or => write!(f, "Or"),
      TokenType::Print => write!(f, "Print"),
      TokenType::Return => write!(f, "Return"),
      TokenType::Super => write!(f, "Super"),
      TokenType::This => write!(f, "This"),
      TokenType::True => write!(f, "True"),
      TokenType::Var => write!(f, "Var"),
      TokenType::While => write!(f, "While"),
      TokenType::Eof => write!(f, "Eof"),
      TokenType::Error => write!(f, "Error"),
    }
  }
}

pub struct Scanner {
  source: Rc<String>,
  start: usize,
  current: usize,
  line: usize,
}

impl Scanner {
  pub fn new(source: &str) -> Scanner {
    Scanner {
      source: Rc::new(source.to_string()),
      start: 0,
      current: 0,
      line: 1,
    }
  }

  pub fn scan_token(&mut self) -> Token {
    self.skip_whitespace();

    self.start = self.current;

    let byte = match self.advance() {
      Some(byte) => byte,
      None => return self.make_token(TokenType::Eof),
    };

    match byte {
      b'(' => self.make_token(TokenType::LeftParen),
      b')' => self.make_token(TokenType::RightParen),
      b'{' => self.make_token(TokenType::LeftBrace),
      b'}' => self.make_token(TokenType::RightBrace),
      b';' => self.make_token(TokenType::Semicolon),
      b',' => self.make_token(TokenType::Comma),
      b'.' => self.make_token(TokenType::Dot),
      b'-' => self.make_token(TokenType::Minus),
      b'+' => self.make_token(TokenType::Plus),
      b'/' => self.make_token(TokenType::Slash),
      b'*' => self.make_token(TokenType::Star),
      b'!' => {
        let ty = if self.expect(b'=') {
          TokenType::BangEqual
        } else {
          TokenType::Bang
        };
        self.make_token(ty)
      }
      b'=' => {
        let ty = if self.expect(b'=') {
          TokenType::EqualEqual
        } else {
          TokenType::Equal
        };
        self.make_token(ty)
      }
      b'<' => {
        let ty = if self.expect(b'=') {
          TokenType::LessEqual
        } else {
          TokenType::Less
        };
        self.make_token(ty)
      }
      b'>' => {
        let ty = if self.expect(b'=') {
          TokenType::GreaterEqual
        } else {
          TokenType::Greater
        };
        self.make_token(ty)
      }
      b'"' => self.string(),
      byte if is_digit(byte) => self.number(),
      byte if is_alpha(byte) => self.identifier(),
      byte => self.error_token(&format!("unexpected char {}", byte as char)),
    }
  }

  fn string(&mut self) -> Token {
    while let Some(byte) = self.peek() {
      if byte == b'"' {
        break;
      }
      if byte == b'\n' {
        self.line += 1;
      }
      self.advance();
    }

    if let None = self.peek() {
      return self.error_token("unterminated string");
    }

    self.advance();

    self.make_token(TokenType::String)
  }

  fn number(&mut self) -> Token {
    loop {
      match self.peek() {
        Some(byte) if is_digit(byte) => {}
        _ => break,
      }
      self.advance();
    }

    match (self.peek(), self.peek_next()) {
      (Some(b'.'), Some(byte)) if is_digit(byte) => {
        self.advance();
        loop {
          match self.peek() {
            Some(byte) if is_digit(byte) => {}
            _ => break,
          }
          self.advance();
        }
      }
      _ => {}
    }

    self.make_token(TokenType::Number)
  }

  fn identifier(&mut self) -> Token {
    loop {
      match self.peek() {
        Some(byte) if is_alpha(byte) || is_digit(byte) => {}
        _ => break,
      }
      self.advance();
    }
    self.make_token(self.identifier_type())
  }

  fn identifier_type(&self) -> TokenType {
    match &self.source[self.start..self.current] {
      "and" => TokenType::And,
      "class" => TokenType::Class,
      "else" => TokenType::Else,
      "if" => TokenType::If,
      "nil" => TokenType::Nil,
      "or" => TokenType::Or,
      "print" => TokenType::Print,
      "return" => TokenType::Return,
      "super" => TokenType::Super,
      "var" => TokenType::Var,
      "while" => TokenType::While,
      "false" => TokenType::False,
      "for" => TokenType::For,
      "fun" => TokenType::Fun,
      "this" => TokenType::This,
      "true" => TokenType::True,
      _ => TokenType::Identifier,
    }
  }

  fn error_token(&self, msg: &str) -> Token {
    Token {
      ty: TokenType::Error,
      source: Rc::new(msg.to_string()),
      start: 0,
      len: msg.len(),
      line: self.line,
    }
  }

  fn make_token(&self, ty: TokenType) -> Token {
    Token {
      ty,
      source: self.source.clone(),
      start: self.start,
      len: self.current - self.start,
      line: self.line,
    }
  }

  fn skip_whitespace(&mut self) {
    loop {
      let byte = match self.peek() {
        Some(byte) => byte,
        None => return,
      };
      match byte {
        b' ' | b'\r' | b'\t' => {
          self.advance();
        }
        b'/' => {
          if let Some(b'/') = self.peek_next() {
            loop {
              match self.peek() {
                Some(b'\n') => break,
                None => break,
                _ => self.advance(),
              };
            }
          } else {
            return;
          }
        }
        b'\n' => {
          self.line += 1;
          self.advance();
        }
        _ => return,
      }
    }
  }

  // This was `match` in clox but `match` is a Rust keyword.
  fn expect(&mut self, expected: u8) -> bool {
    match self.peek() {
      Some(byte) if byte == expected => {
        self.advance();
        true
      }
      _ => false,
    }
  }

  fn advance(&mut self) -> Option<u8> {
    self.current += 1;
    self.source.as_bytes().get(self.current - 1).copied()
  }

  fn peek_next(&self) -> Option<u8> {
    self.source.as_bytes().get(self.current + 1).copied()
  }

  fn peek(&self) -> Option<u8> {
    self.source.as_bytes().get(self.current).copied()
  }

  pub fn tmp_scan(&mut self) {
    let mut line = 0;
    loop {
      let token = self.scan_token();
      if token.line != line {
        print!("{:>4} ", token.line);
        line = token.line;
      } else {
        print!("   | ");
      }
      if token.ty == TokenType::Eof {
        println!("{:>10}", token.ty);
      } else {
        println!("{:>10} '{}'", token.ty, token.as_str(),);
      }
      if token.ty == TokenType::Eof {
        break;
      }
    }
  }
}

fn is_alpha(byte: u8) -> bool {
  (byte >= b'a' && byte <= b'z')
    || (byte >= b'A' && byte <= b'Z')
    || byte == b'_'
}

fn is_digit(byte: u8) -> bool {
  byte >= b'0' && byte <= b'9'
}
