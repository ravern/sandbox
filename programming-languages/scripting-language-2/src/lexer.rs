use crate::error::Error;
use crate::pos::Pos;
use crate::result::Result;
use crate::token::{Token, TokenKind};
use std::iter::Peekable;

pub struct Lexer<I>
where
  I: Iterator<Item = char>,
{
  path: String,
  chars: Peekable<I>,
  pos: Pos,
  ended: bool,
}

impl<I> Lexer<I>
where
  I: Iterator<Item = char>,
{
  pub fn new<S>(path: S, chars: I) -> Lexer<I>
  where
    S: Into<String>,
  {
    Lexer {
      chars: chars.peekable(),
      path: path.into(),
      pos: Pos::new(),
      ended: false,
    }
  }

  pub fn lex(self) -> Result<Vec<Token>> {
    self.collect()
  }

  fn lex_token(&mut self) -> Result<Token> {
    loop {
      let pos = self.pos;
      let kind = match self.chars.peek().cloned() {
        Some(c) if c.is_whitespace() => {
          self.advance();
          // self.pos.column += 1;
          continue;
        }
        Some('+') => Ok(self.lex_operator(TokenKind::Add)),
        Some('-') => Ok(self.lex_operator(TokenKind::Subtract)),
        Some('*') => Ok(self.lex_operator(TokenKind::Multiply)),
        Some('^') => Ok(self.lex_operator(TokenKind::Power)),
        Some('/') => Ok(self.lex_operator(TokenKind::Divide)),
        Some('(') => Ok(self.lex_operator(TokenKind::LeftParen)),
        Some(')') => Ok(self.lex_operator(TokenKind::RightParen)),
        Some('=') => Ok(self.lex_operator(TokenKind::Assign)),
        Some(c) if c.is_digit(10) => Ok(self.lex_number()),
        Some(c) if c.is_alphabetic() => Ok(self.lex_identifier()),
        Some(c) => Err(self.new_unexpected_parse_error(&format!("\"{}\"", c))),
        None => Ok(self.lex_end_of_file()),
      }?;
      return Ok(Token { pos, kind });
    }
  }

  fn lex_operator(&mut self, kind: TokenKind) -> TokenKind {
    self.advance();
    kind
  }

  fn lex_number(&mut self) -> TokenKind {
    let mut token: Vec<char> = vec![];
    let mut decimaled = false;
    loop {
      match self.chars.peek().cloned() {
        Some('.') if !decimaled => decimaled = true,
        Some(c) if !c.is_digit(10) => return self.new_number_token_kind(&token),
        None => return self.new_number_token_kind(&token),
        _ => {}
      }
      token.push(self.advance().unwrap());
    }
  }

  fn lex_identifier(&mut self) -> TokenKind {
    let mut token: Vec<char> = vec![];
    loop {
      match self.chars.peek().cloned() {
        Some(c) if !c.is_alphabetic() || c == '_' => {
          return self.new_identifier_token_kind(&token);
        }
        None => return self.new_identifier_token_kind(&token),
        _ => {}
      }
      token.push(self.advance().unwrap())
    }
  }

  fn lex_end_of_file(&mut self) -> TokenKind {
    self.ended = true;
    self.advance();
    TokenKind::EndOfFile
  }

  fn new_number_token_kind(&mut self, token: &[char]) -> TokenKind {
    let token = token
      .into_iter()
      .collect::<String>()
      .parse::<f32>()
      .unwrap();
    TokenKind::Number(token)
  }

  fn new_identifier_token_kind(&mut self, token: &[char]) -> TokenKind {
    let token = token.into_iter().collect::<String>();
    match token.as_ref() {
      "and" => TokenKind::And,
      "or" => TokenKind::Or,
      "not" => TokenKind::Not,
      _ => TokenKind::Identifier(token),
    }
  }

  fn new_unexpected_parse_error(&mut self, unexpected: &str) -> Error {
    let pos = self.pos;
    self.advance();
    Error::new_parse(format!("Unexpected {}", unexpected), self.path.clone(), pos)
  }

  fn advance(&mut self) -> Option<char> {
    self.pos.column += 1;
    self.chars.next()
  }
}

impl<I> Iterator for Lexer<I>
where
  I: Iterator<Item = char>,
{
  type Item = Result<Token>;

  fn next(&mut self) -> Option<Result<Token>> {
    if self.ended {
      None
    } else {
      Some(self.lex_token())
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::pos::Pos;

  #[test]
  fn test_numbers() {
    let program = "123 + 456";
    let lexer = Lexer::new("test", program.chars());
    let tokens = lexer.collect::<Result<Vec<Token>>>().unwrap();
    assert_eq!(
      tokens,
      vec![
        Token {
          pos: Pos { line: 1, column: 1 },
          kind: TokenKind::Number(123.0),
        },
        Token {
          pos: Pos { line: 1, column: 5 },
          kind: TokenKind::Add,
        },
        Token {
          pos: Pos { line: 1, column: 7 },
          kind: TokenKind::Number(456.0),
        },
        Token {
          pos: Pos {
            line: 1,
            column: 10
          },
          kind: TokenKind::EndOfFile,
        }
      ]
    )
  }

  #[test]
  fn test_kinds_symbols() {
    let program = "+-*/^()";
    let lexer = Lexer::new("test", program.chars());
    let token_kinds = collect_token_kinds(lexer);
    assert_eq!(
      token_kinds,
      vec![
        TokenKind::Add,
        TokenKind::Subtract,
        TokenKind::Multiply,
        TokenKind::Divide,
        TokenKind::Power,
        TokenKind::LeftParen,
        TokenKind::RightParen,
        TokenKind::EndOfFile
      ]
    );
  }

  #[test]
  fn test_kinds_numbers() {
    let program = "1234.5678";
    let lexer = Lexer::new("test", program.chars());
    let token_kinds = collect_token_kinds(lexer);
    assert_eq!(
      token_kinds,
      vec![TokenKind::Number(1234.5678), TokenKind::EndOfFile]
    );
  }

  #[test]
  fn test_kinds_identifiers() {
    let program = "one + two";
    let lexer = Lexer::new("test", program.chars());
    let token_kinds = collect_token_kinds(lexer);
    assert_eq!(
      token_kinds,
      vec![
        TokenKind::Identifier("one".into()),
        TokenKind::Add,
        TokenKind::Identifier("two".into()),
        TokenKind::EndOfFile
      ]
    );
  }

  #[test]
  fn test_kinds_keywords() {
    let program = "one and two";
    let lexer = Lexer::new("test", program.chars());
    let token_kinds = collect_token_kinds(lexer);
    assert_eq!(
      token_kinds,
      vec![
        TokenKind::Identifier("one".into()),
        TokenKind::And,
        TokenKind::Identifier("two".into()),
        TokenKind::EndOfFile
      ]
    );
  }

  #[test]
  fn test_kinds_whitespace() {
    let program = "1 + 1";
    let lexer = Lexer::new("test", program.chars());
    let token_kinds = collect_token_kinds(lexer);
    assert_eq!(
      token_kinds,
      vec![
        TokenKind::Number(1.0),
        TokenKind::Add,
        TokenKind::Number(1.0),
        TokenKind::EndOfFile
      ]
    );
  }

  #[test]
  fn test_kinds_end_of_file() {
    let program = "";
    let lexer = Lexer::new("test", program.chars());
    let token_kinds = collect_token_kinds(lexer);
    assert_eq!(token_kinds, vec![TokenKind::EndOfFile]);
  }

  fn collect_token_kinds<I>(lexer: Lexer<I>) -> Vec<TokenKind>
  where
    I: Iterator<Item = char>,
  {
    lexer
      .collect::<Result<Vec<Token>>>()
      .unwrap()
      .into_iter()
      .map(|token| token.kind)
      .collect::<Vec<TokenKind>>()
  }
}
