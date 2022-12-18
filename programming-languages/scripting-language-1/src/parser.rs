use ast::*;
use error::Error;
use result::Result;
use std::iter::Peekable;
use token::{Token, TokenKind};

pub struct Parser<I>
where
  I: Iterator<Item = Result<Token>>,
{
  tokens: Peekable<I>,
}

impl<I> Parser<I>
where
  I: Iterator<Item = Result<Token>>,
{
  pub fn new(tokens: I) -> Parser<I> {
    Parser {
      tokens: tokens.peekable(),
    }
  }

  pub fn parse(&mut self) -> ::std::result::Result<Program, Vec<Error>> {
    match self.parse_expression() {
      Ok(expression) => Ok(Program::from_expression(expression)),
      Err(err) => Err(vec![err]),
    }
  }

  fn parse_expression(&mut self) -> Result<Expression> {
    let operation = self.parse_logical_operation()?;

    match self.tokens.peek() {
      Some(&token) => match token?.kind {
        TokenKind::Assign => {}
        _ => return Ok(Expression::from_operation(operation)),
      },
      None => return Ok(Expression::from_operation(operation)),
    }
    self.tokens.next();

    match operation.kind {
      OperationKind::Term(term) => match term.kind {
        TermKind::Identifier(identifier) => {
          return Ok(Expression::from_assignment(
            identifier,
            self.parse_expression()?,
          ));
        }
      },
      _ => {}
    }

    Err(Error::Parse {
      span: operation.span,
    })
  }

  fn parse_logical_operation(&mut self) -> Result<Operation> {
    let operation = self.parse_add_operation()?;

    let token = self.tokens.peek();
    match token {
      Some(&token) => match token?.kind {
        TokenKind::
      }
    }
  }
}
