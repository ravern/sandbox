use crate::chunk::{Chunk, Instruction, Value};
use crate::error::Error;
use crate::result::Result;
use crate::token::{Token, TokenKind};
use std::iter::Peekable;

pub struct Parser<I>
where
  I: Iterator<Item = Token>,
{
  path: String,
  tokens: Peekable<I>,
}

impl<I> Parser<I>
where
  I: Iterator<Item = Token>,
{
  pub fn new<S>(path: S, tokens: I) -> Parser<I>
  where
    S: Into<String>,
  {
    Parser {
      path: path.into(),
      tokens: tokens.peekable(),
    }
  }

  pub fn parse(self) -> Vec<Result<Chunk>> {
    self.collect()
  }

  fn parse_declaration(&mut self) -> Result<Chunk> {
    let mut chunk = Chunk::new(self.path.clone());
    self.parse_expression(&mut chunk)?;
    Ok(chunk)
  }

  fn parse_expression(&mut self, chunk: &mut Chunk) -> Result<()> {
    self.parse_assignment_operation(chunk)
  }

  fn parse_assignment_operation(&mut self, chunk: &mut Chunk) -> Result<()> {
    self.parse_logical_operation(chunk)?;

    match self.tokens.peek() {
      Some(token) => match token.kind {
        TokenKind::Assign => {}
        _ => return Ok(()),
      },
      None => return Ok(()),
    }
    let operator = self.tokens.next().unwrap();

    // Obtain the `index` of the identifier to push as `Instruction::Store`.
    let instruction = chunk.instructions.pop().unwrap();
    let index = match instruction {
      Instruction::Load(index) => index,
      _ => {
        return Err(Error::new_parse(
          "Cannot assign to non-identifier",
          self.path.clone(),
          *chunk.poses.last().unwrap(),
        ));
      }
    };

    self.parse_assignment_operation(chunk)?;
    chunk.push(Instruction::Store(index), operator.pos);
    Ok(())
  }

  fn parse_logical_operation(&mut self, chunk: &mut Chunk) -> Result<()> {
    self.parse_add_operation(chunk)?;

    loop {
      match self.tokens.peek() {
        Some(token) => match token.kind {
          TokenKind::And => {}
          TokenKind::Or => {}
          _ => return Ok(()),
        },
        None => return Ok(()),
      }
      let operator = self.tokens.next().unwrap();

      self.parse_add_operation(chunk)?;
      chunk.push(Instruction::from_token(&operator).unwrap(), operator.pos);
    }
  }

  fn parse_add_operation(&mut self, chunk: &mut Chunk) -> Result<()> {
    self.parse_multiply_operation(chunk)?;

    loop {
      match self.tokens.peek() {
        Some(token) => match token.kind {
          TokenKind::Add => {}
          TokenKind::Subtract => {}
          _ => return Ok(()),
        },
        None => return Ok(()),
      }
      let operator = self.tokens.next().unwrap();

      self.parse_multiply_operation(chunk)?;
      chunk.push(Instruction::from_token(&operator).unwrap(), operator.pos);
    }
  }

  fn parse_multiply_operation(&mut self, chunk: &mut Chunk) -> Result<()> {
    self.parse_power_operation(chunk)?;

    loop {
      match self.tokens.peek() {
        Some(token) => match token.kind {
          TokenKind::Multiply => {}
          TokenKind::Divide => {}
          _ => return Ok(()),
        },
        None => return Ok(()),
      }
      let operator = self.tokens.next().unwrap();

      self.parse_power_operation(chunk)?;
      chunk.push(Instruction::from_token(&operator).unwrap(), operator.pos);
    }
  }

  fn parse_power_operation(&mut self, chunk: &mut Chunk) -> Result<()> {
    self.parse_unary_operation(chunk)?;

    match self.tokens.peek() {
      Some(token) => match token.kind {
        TokenKind::Power => {}
        _ => return Ok(()),
      },
      None => return Ok(()),
    }
    let operator = self.tokens.next().unwrap();

    self.parse_power_operation(chunk)?;
    chunk.push(Instruction::from_token(&operator).unwrap(), operator.pos);
    Ok(())
  }

  fn parse_unary_operation(&mut self, chunk: &mut Chunk) -> Result<()> {
    let mut negate = false;
    match self.tokens.peek() {
      Some(token) => match token.kind {
        TokenKind::Not => {}
        TokenKind::Subtract => negate = true,
        _ => return self.parse_term(chunk),
      },
      None => return self.parse_term(chunk),
    }
    let operator = self.tokens.next().unwrap();

    self.parse_unary_operation(chunk)?;
    if negate {
      chunk.push(Instruction::Negate, operator.pos);
    } else {
      chunk.push(Instruction::from_token(&operator).unwrap(), operator.pos);
    }
    Ok(())
  }

  fn parse_term(&mut self, chunk: &mut Chunk) -> Result<()> {
    let token = self.tokens.next().unwrap();
    match token.kind {
      TokenKind::Identifier(identifier) => {
        chunk.push_load(identifier, token.pos);
        Ok(())
      }
      _ => match Value::from_token(&token) {
        Some(value) => {
          chunk.push_push(value, token.pos);
          return Ok(());
        }
        None => Err(Error::new_parse(
          format!("Unexpected \"{}\"", token),
          self.path.clone(),
          token.pos,
        )),
      },
    }
  }
}

impl<I> Iterator for Parser<I>
where
  I: Iterator<Item = Token>,
{
  type Item = Result<Chunk>;

  fn next(&mut self) -> Option<Result<Chunk>> {
    match self.tokens.peek() {
      Some(token) => match token.kind {
        TokenKind::EndOfFile => None,
        _ => Some(self.parse_declaration()),
      },
      None => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::pos::Pos;
  use maplit::hashmap;

  #[test]
  fn test_operator_precedence() {
    let token_kinds = vec![
      TokenKind::Number(1.0),
      TokenKind::And,
      TokenKind::Number(2.0),
      TokenKind::Or,
      TokenKind::Number(3.0),
      TokenKind::Add,
      TokenKind::Number(4.0),
      TokenKind::Subtract,
      TokenKind::Number(5.0),
      TokenKind::Multiply,
      TokenKind::Number(6.0),
      TokenKind::Divide,
      TokenKind::Number(7.0),
      TokenKind::Power,
      TokenKind::Number(8.0),
      TokenKind::Power,
      TokenKind::Number(9.0),
    ];
    let mut parser = Parser::new("test", tokens_from_token_kinds(token_kinds).into_iter());
    let chunk = parser.next().unwrap().unwrap();
    assert_eq!(
      chunk.constants.clone(),
      vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
        Value::Number(4.0),
        Value::Number(5.0),
        Value::Number(6.0),
        Value::Number(7.0),
        Value::Number(8.0),
        Value::Number(9.0),
      ],
    );
    assert_eq!(
      chunk.instructions.clone(),
      vec![
        Instruction::Push(0),
        Instruction::Push(1),
        Instruction::And,
        Instruction::Push(2),
        Instruction::Push(3),
        Instruction::Add,
        Instruction::Push(4),
        Instruction::Push(5),
        Instruction::Multiply,
        Instruction::Push(6),
        Instruction::Push(7),
        Instruction::Push(8),
        Instruction::Power,
        Instruction::Power,
        Instruction::Divide,
        Instruction::Subtract,
        Instruction::Or,
      ],
    );
  }

  #[test]
  fn test_assignment() {
    let token_kinds = vec![
      TokenKind::Identifier("one".into()),
      TokenKind::Assign,
      TokenKind::Number(1.0),
    ];
    let mut parser = Parser::new("test", tokens_from_token_kinds(token_kinds).into_iter());
    let chunk = parser.next().unwrap().unwrap();
    assert_eq!(chunk.constants.clone(), vec![Value::Number(1.0)],);
    assert_eq!(chunk.identifiers.clone(), hashmap! { "one".into() => 0 });
    assert_eq!(
      chunk.instructions.clone(),
      vec![Instruction::Push(0), Instruction::Store(0)],
    );
  }

  #[test]
  fn test_logical() {
    let token_kinds = vec![
      TokenKind::Number(1.0),
      TokenKind::And,
      TokenKind::Number(1.0),
    ];
    let mut parser = Parser::new("test", tokens_from_token_kinds(token_kinds).into_iter());
    let chunk = parser.next().unwrap().unwrap();
    assert_eq!(
      chunk.constants.clone(),
      vec![Value::Number(1.0), Value::Number(1.0)],
    );
    assert_eq!(
      chunk.instructions.clone(),
      vec![Instruction::Push(0), Instruction::Push(1), Instruction::And],
    );
  }

  #[test]
  fn test_addition() {
    let token_kinds = vec![
      TokenKind::Number(1.0),
      TokenKind::Add,
      TokenKind::Number(1.0),
    ];
    let mut parser = Parser::new("test", tokens_from_token_kinds(token_kinds).into_iter());
    let chunk = parser.next().unwrap().unwrap();
    assert_eq!(
      chunk.constants.clone(),
      vec![Value::Number(1.0), Value::Number(1.0)],
    );
    assert_eq!(
      chunk.instructions.clone(),
      vec![Instruction::Push(0), Instruction::Push(1), Instruction::Add]
    );
  }

  #[test]
  fn test_multiply() {
    let token_kinds = vec![
      TokenKind::Number(1.0),
      TokenKind::Multiply,
      TokenKind::Number(1.0),
    ];
    let mut parser = Parser::new("test", tokens_from_token_kinds(token_kinds).into_iter());
    let chunk = parser.next().unwrap().unwrap();
    assert_eq!(
      chunk.constants.clone(),
      vec![Value::Number(1.0), Value::Number(1.0)],
    );
    assert_eq!(
      chunk.instructions.clone(),
      vec![
        Instruction::Push(0),
        Instruction::Push(1),
        Instruction::Multiply
      ]
    );
  }

  #[test]
  fn test_power() {
    let token_kinds = vec![
      TokenKind::Number(1.0),
      TokenKind::Power,
      TokenKind::Number(1.0),
    ];
    let mut parser = Parser::new("test", tokens_from_token_kinds(token_kinds).into_iter());
    let chunk = parser.next().unwrap().unwrap();
    assert_eq!(
      chunk.constants.clone(),
      vec![Value::Number(1.0), Value::Number(1.0)],
    );
    assert_eq!(
      chunk.instructions.clone(),
      vec![
        Instruction::Push(0),
        Instruction::Push(1),
        Instruction::Power
      ]
    );
  }

  fn tokens_from_token_kinds(kinds: Vec<TokenKind>) -> Vec<Token> {
    kinds
      .into_iter()
      .map(|kind| Token {
        pos: Pos::new(),
        kind,
      }).collect()
  }
}
