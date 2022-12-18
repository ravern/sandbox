use {inflections::case::is_pascal_case, intern::Intern};

use crate::{
  ast::*,
  error::{CompileError, ParseError, VerifyError},
  lex::{Lexeme, Lexer, Token},
  source::{Source, Span},
};

pub struct Parser {
  source: Source,
  lexer: Lexer,
  current: Option<Lexeme>,
}

impl Parser {
  pub fn new(source: Source) -> Self {
    Self {
      lexer: Lexer::new(source.clone()),
      source,
      current: None,
    }
  }

  pub fn parse(mut self) -> Result<Module, CompileError> {
    let module = self.module()?;
    self.expect([Token::End])?;
    Ok(module)
  }

  fn module(&mut self) -> Result<Module, CompileError> {
    Ok(Module {
      body: self.block_expr([Token::End])?,
    })
  }

  fn block_expr<const N: usize>(
    &mut self,
    terminators: [Token; N],
  ) -> Result<Expr, CompileError> {
    let span = self.peek()?.span().clone();
    let mut span = Span::new(span.source().clone(), span.start(), span.end());

    let mut exprs = Vec::new();
    let mut has_semi = true;

    while !terminators.contains(&self.peek()?.token()) {
      let expr = self.expr()?;

      span = Span::combine(&span, &expr.span());

      if let Token::Semicolon = self.peek()?.token() {
        self.expect([Token::Semicolon]).unwrap();
        has_semi = true;
      } else {
        match &expr {
          Expr::If(_) | Expr::While(_) | Expr::For(_) | Expr::Case(_) => {
            has_semi = false
          }
          _ => {
            if !terminators.contains(&self.peek()?.token()) {
              return Err(self.build_error(Some(terminators)));
            } else {
              has_semi = false;
            }
          }
        }
      }

      exprs.push(expr);
    }

    Ok(Expr::Block(BlockExpr {
      span,
      exprs,
      has_semi,
    }))
  }

  fn expr(&mut self) -> Result<Expr, CompileError> {
    match self.peek()?.token() {
      Token::Let => self.bind_expr(),
      Token::If => self.if_expr(),
      Token::Case => self.case_expr(),
      Token::For => self.for_expr(),
      Token::While => self.while_expr(),
      _ => self.pratt_expr(0),
    }
  }

  fn bind_expr(&mut self) -> Result<Expr, CompileError> {
    self.expect([Token::Let])?;

    if let Expr::Assign(assign_expr) = self.expr()? {
      if let AssignExprAssignee::Pat(pat) = assign_expr.assignee {
        Ok(Expr::Bind(BindExpr {
          bindee: pat,
          value: assign_expr.value,
        }))
      } else {
        unimplemented!();
      }
    } else {
      unimplemented!();
    }
  }

  fn if_expr(&mut self) -> Result<Expr, CompileError> {
    let if_lexeme = self.expect([Token::If]).unwrap();

    let condition = self.expr()?;

    self.expect([Token::OpenBrace])?;
    let body = self.block_expr([Token::CloseBrace])?;
    let close_brace_token = self.expect([Token::CloseBrace]).unwrap();

    let otherwise = if let Token::Else = self.peek()?.token() {
      self
        .expect([Token::Else])
        .expect("`self.peek` returned else token");
      match self.peek()?.token() {
        Token::If => Some(self.if_expr()?),
        Token::OpenBrace => {
          self
            .expect([Token::OpenBrace])
            .expect("`self.peek` returns open brace token");
          let body = self.block_expr([Token::CloseBrace])?;
          self
            .expect([Token::CloseBrace])
            .expect("`self.block_expr` ended on close brace");
          Some(body)
        }
        _ => return Err(self.build_error(Some([Token::If, Token::OpenBrace]))),
      }
    } else {
      None
    };

    Ok(Expr::If(IfExpr {
      span: Span::combine(
        if_lexeme.span(),
        &otherwise
          .as_ref()
          .map(|otherwise| otherwise.span())
          .unwrap_or(close_brace_token.span().clone()),
      ),
      condition: Box::new(condition),
      body: Box::new(body),
      otherwise: otherwise.map(Box::new),
    }))
  }

  fn case_expr(&mut self) -> Result<Expr, CompileError> {
    let case_lexeme = self.expect([Token::Case]).unwrap();

    let subject = self.expr()?;

    self.expect([Token::OpenBrace])?;
    let mut arms = Vec::new();
    while self.peek()?.token() != Token::CloseBrace {
      let pat = self.pat()?;

      self.expect([Token::Arrow])?;

      let expr = if let Token::OpenBrace = self.peek()?.token() {
        self.expect([Token::OpenBrace]).unwrap();
        let expr = self.block_expr([Token::CloseBrace])?;
        self.expect([Token::CloseBrace]).unwrap();
        expr
      } else {
        let expr = self.expr()?;
        self.expect([Token::Comma])?;
        expr
      };

      arms.push((pat, expr))
    }
    let close_brace_lexeme = self
      .expect([Token::CloseBrace])
      .expect("earlier `while` ended on close brace token");

    Ok(Expr::Case(CaseExpr {
      span: Span::combine(case_lexeme.span(), close_brace_lexeme.span()),
      subject: Box::new(subject),
      arms,
    }))
  }

  fn for_expr(&mut self) -> Result<Expr, CompileError> {
    let for_lexeme = self.expect([Token::For]).unwrap();

    let item = self.pat()?;

    self.expect([Token::In])?;

    let iterator = self.expr()?;

    self.expect([Token::OpenBrace])?;

    let body = self.block_expr([Token::CloseBrace])?;

    let close_brace_lexeme = self.expect([Token::CloseBrace]).unwrap();

    Ok(Expr::For(ForExpr {
      span: Span::combine(for_lexeme.span(), close_brace_lexeme.span()),
      item,
      iterator: Box::new(iterator),
      body: Box::new(body),
    }))
  }

  fn while_expr(&mut self) -> Result<Expr, CompileError> {
    let while_expr = self.expect([Token::While]).unwrap();

    let condition = self.expr()?;

    self.expect([Token::OpenBrace])?;

    let body = self.block_expr([Token::CloseBrace])?;

    let close_brace_lexeme = self.expect([Token::CloseBrace]).unwrap();

    Ok(Expr::While(WhileExpr {
      span: Span::combine(while_expr.span(), close_brace_lexeme.span()),
      condition: Box::new(condition),
      body: Box::new(body),
    }))
  }

  fn pratt_expr(&mut self, min_power: u8) -> Result<Expr, CompileError> {
    let mut left = match self.peek()?.token() {
      Token::OpenParen => self.group_or_lambda_expr()?,
      Token::OpenBrace => self.map_expr()?,
      Token::OpenBracket => self.array_expr()?,
      Token::Number | Token::Bool | Token::String | Token::Null => {
        Expr::Lit(self.lit()?)
      }
      Token::Ident => Expr::Ident(self.ident()?),
      _ => self.prefix_expr()?,
    };

    loop {
      if let Token::Semicolon | Token::End = self.peek()?.token() {
        break;
      }

      if let Some(power) = postfix_binding_power(self.peek()?.token()) {
        if power < min_power {
          break;
        }

        left = match self.advance().unwrap().token() {
          Token::OpenBracket => {
            let field = AccessExprField::Expr(Box::new(self.expr()?));
            self.expect([Token::CloseBracket])?;
            Expr::Access(AccessExpr {
              receiver: Box::new(left),
              field,
            })
          }
          Token::OpenParen => {
            let arguments = self.call_expr_arguments()?;
            let close_paren_lexeme = self.expect([Token::CloseParen]).unwrap();
            match &left {
              Expr::Ident(ident) if is_pascal_case(ident.content) => {
                if arguments.len() == 1 {
                  let mut arguments = arguments;
                  let argument = arguments.pop().unwrap();
                  if let CallExprArgument::Expr(expr) = argument {
                    Expr::Tag(TagExpr {
                      span: Span::combine(
                        &left.span(),
                        close_paren_lexeme.span(),
                      ),
                      tag: ident.clone(),
                      expr: Box::new(expr),
                    })
                  } else {
                    let span =
                      Span::combine(&left.span(), close_paren_lexeme.span());
                    return Err(CompileError::Verify(
                      VerifyError::multiple_tag_arguments(span),
                    ));
                  }
                } else {
                  let span =
                    Span::combine(&left.span(), close_paren_lexeme.span());
                  return Err(CompileError::Verify(
                    VerifyError::multiple_tag_arguments(span),
                  ));
                }
              }
              _ => Expr::Call(CallExpr {
                receiver: Box::new(left),
                arguments,
              }),
            }
          }
          _ => unreachable!(),
        };

        continue;
      }

      if let Some((left_power, right_power)) =
        infix_binding_power(self.peek()?.token())
      {
        if left_power < min_power {
          break;
        }

        left = match self.advance().unwrap().token() {
          Token::Dot => {
            let field = AccessExprField::Ident(self.ident()?);
            Expr::Access(AccessExpr {
              receiver: Box::new(left),
              field,
            })
          }
          Token::Equal => {
            let value = self.expr()?;
            let span = left.span().clone();
            Expr::Assign(AssignExpr {
              assignee: AssignExprAssignee::from_expr(left).ok_or(
                CompileError::Verify(VerifyError::invalid_assignee(span)),
              )?,
              value: Box::new(value),
            })
          }
          token => Expr::Binary(BinaryExpr {
            op: BinaryOp::from_token(token).unwrap(),
            left: Box::new(left),
            right: Box::new(self.pratt_expr(right_power)?),
          }),
        };

        continue;
      }

      break;
    }

    Ok(left)
  }

  fn prefix_expr(&mut self) -> Result<Expr, CompileError> {
    if let Some(power) = prefix_binding_power(self.peek()?.token()) {
      let op = self.advance().unwrap();
      let operand = self.pratt_expr(power)?;
      Ok(Expr::Unary(UnaryExpr {
        span: Span::combine(op.span(), &operand.span()),
        op: UnaryOp::from_token(op.token()).unwrap(),
        operand: Box::new(operand),
      }))
    } else {
      Err(self.build_error::<0>(None))
    }
  }

  fn call_expr_arguments(
    &mut self,
  ) -> Result<Vec<CallExprArgument>, CompileError> {
    let mut arguments = Vec::new();
    while self.peek()?.token() != Token::CloseParen {
      let argument = match self.peek()?.token() {
        Token::DotDot => {
          self.expect([Token::DotDot]).unwrap();
          CallExprArgument::Spread(self.expr()?)
        }
        _ => CallExprArgument::Expr(self.expr()?),
      };

      arguments.push(argument);

      match self.peek()?.token() {
        Token::Comma => {
          self.expect([Token::Comma]).unwrap();
        }
        Token::CloseParen => {}
        _ => {
          return Err(self.build_error(Some([Token::Comma, Token::CloseParen])))
        }
      }
    }
    Ok(arguments)
  }

  fn group_or_lambda_expr(&mut self) -> Result<Expr, CompileError> {
    let open_paren_lexeme = self.peek()?;
    assert_eq!(open_paren_lexeme.token(), Token::OpenParen);

    // Generate the span from the open paren to the end of the source.
    let span = Span::new(
      self.source.clone(),
      open_paren_lexeme.span().start(),
      self.source.content().len(),
    );

    // Create the source for the above span.
    let source = Source::from_str(span.content(), self.source.path());

    // Try parsing as a lambda first. If that doesn't work, we parse it as
    // a group. If that doesn't work then just return an error.
    match (
      Parser::new(source.clone()).lambda_expr_head(),
      Parser::new(source.clone()).group_expr(),
    ) {
      (Ok(_), _) => self.lambda_expr(),
      (Err(_), Ok(_)) => self.group_expr(),
      (Err(_), Err(_)) => self.lambda_expr(),
    }
  }

  fn group_expr(&mut self) -> Result<Expr, CompileError> {
    self.expect([Token::OpenParen]).unwrap();
    let expr = self.expr()?;
    self.expect([Token::CloseParen])?;
    Ok(expr)
  }

  fn lambda_expr(&mut self) -> Result<Expr, CompileError> {
    let (parameters, lambda_head_span) = self.lambda_expr_head()?;

    let (body, body_span) = match self.peek()?.token() {
      Token::OpenBrace => {
        let open_brace_lexeme = self.expect([Token::OpenBrace]).unwrap();
        let body = self.block_expr([Token::CloseBrace])?;
        let close_brace_lexeme = self.expect([Token::CloseBrace]).unwrap();
        (
          body,
          Span::combine(open_brace_lexeme.span(), close_brace_lexeme.span()),
        )
      }
      _ => {
        let expr = self.expr()?;
        let span = expr.span();
        (expr, span)
      }
    };

    Ok(Expr::Lambda(LambdaExpr {
      span: Span::combine(&lambda_head_span, &body_span),
      parameters,
      body: Box::new(body),
    }))
  }

  fn lambda_expr_head(
    &mut self,
  ) -> Result<(Vec<LambdaExprParameter>, Span), CompileError> {
    let open_paren_lexeme = self.expect([Token::OpenParen]).unwrap();

    let mut parameters = Vec::new();
    while self.peek()?.token() != Token::CloseParen {
      let parameter = match self.peek()?.token() {
        Token::DotDot => {
          self.expect([Token::DotDot]).unwrap();
          LambdaExprParameter::Spread(self.ident()?)
        }
        _ => LambdaExprParameter::Pat(self.pat()?),
      };

      parameters.push(parameter);

      match self.peek()?.token() {
        Token::Comma => {
          self
            .expect([Token::Comma])
            .expect("`self.peek` returned close paren token");
        }
        Token::CloseParen => {}
        _ => {
          return Err(
            self.build_error(Some([Token::Comma, Token::CloseParen])),
          );
        }
      }
    }

    self
      .expect([Token::CloseParen])
      .expect("`self.peek` returned close paren token");

    let arrow_lexeme = self.expect([Token::Arrow])?;

    Ok((
      parameters,
      Span::combine(open_paren_lexeme.span(), arrow_lexeme.span()),
    ))
  }

  fn map_expr(&mut self) -> Result<Expr, CompileError> {
    let open_brace_lexeme = self.expect([Token::OpenBrace]).unwrap();

    let mut pairs = Vec::new();
    while self.peek()?.token() != Token::CloseBrace {
      let pair = match self.peek()?.token() {
        Token::DotDot => {
          self.expect([Token::DotDot]).unwrap();
          let expr = self.expr()?;
          MapExprPair::Spread(expr)
        }
        token => {
          if let Token::OpenBracket = token {
            self.expect([Token::OpenBracket]).unwrap();
            let field = self.expr()?;
            self.expect([Token::CloseBracket])?;
            self.expect([Token::Colon])?;
            let value = self.expr()?;
            MapExprPair::Expr(field, value)
          } else {
            let field = self.ident()?;
            let value = if let Token::Colon = self.peek()?.token() {
              self.expect([Token::Colon]).unwrap();
              self.expr()?
            } else {
              Expr::Ident(field.clone())
            };
            MapExprPair::Ident(field, value)
          }
        }
      };

      pairs.push(pair);

      match self.peek()?.token() {
        Token::Comma => {
          self.expect([Token::Comma]).unwrap();
        }
        Token::CloseBrace => {}
        _ => {
          return Err(self.build_error(Some([
            Token::Comma,
            Token::CloseBrace,
            Token::Colon,
          ])));
        }
      }
    }

    let close_brace_lexeme = self.expect([Token::CloseBrace]).unwrap();

    Ok(Expr::Map(MapExpr {
      span: Span::combine(open_brace_lexeme.span(), close_brace_lexeme.span()),
      pairs,
    }))
  }

  fn array_expr(&mut self) -> Result<Expr, CompileError> {
    let open_bracket_lexeme = self.expect([Token::OpenBracket]).unwrap();

    let mut items = Vec::new();
    while self.peek()?.token() != Token::CloseBracket {
      let item = match self.peek()?.token() {
        Token::DotDot => {
          self.expect([Token::DotDot]).unwrap();
          let expr = self.expr()?;
          ArrayExprItem::Spread(expr)
        }
        _ => ArrayExprItem::Expr(self.expr()?),
      };

      items.push(item);

      match self.peek()?.token() {
        Token::Comma => {
          self.expect([Token::Comma]).unwrap();
        }
        Token::CloseBracket => {}
        _ => {
          return Err(
            self.build_error(Some([Token::Comma, Token::CloseBracket])),
          );
        }
      }
    }

    let close_bracket_lexeme = self.expect([Token::CloseBracket]).unwrap();

    Ok(Expr::Array(ArrayExpr {
      span: Span::combine(
        open_bracket_lexeme.span(),
        close_bracket_lexeme.span(),
      ),
      items,
    }))
  }

  fn ident(&mut self) -> Result<Ident, CompileError> {
    let lexeme = self.expect([Token::Ident]).unwrap();
    let span = lexeme.span();
    Ok(Ident {
      span: span.clone(),
      content: span.content().intern(),
    })
  }

  fn lit(&mut self) -> Result<Lit, CompileError> {
    let lit = match self.peek()?.token() {
      Token::Number => {
        let lexeme = self.expect([Token::Number]).unwrap();
        let number = lexeme
          .span()
          .content()
          .parse()
          .expect("invalid number was lexed");
        Lit::Number(NumberLit {
          span: lexeme.span().clone(),
          number,
        })
      }
      Token::Bool => {
        let lexeme = self.expect([Token::Bool]).unwrap();
        let bool = lexeme
          .span()
          .content()
          .parse()
          .expect("invalid bool was lexed");
        Lit::Bool(BoolLit {
          span: lexeme.span().clone(),
          bool,
        })
      }
      Token::String => {
        let lexeme = self.expect([Token::String]).unwrap();
        let content = lexeme.span().content();
        let string = &content[1..content.len() - 1].intern();
        Lit::String(StringLit {
          span: lexeme.span().clone(),
          string,
        })
      }
      Token::Null => {
        let lexeme = self.expect([Token::Null]).unwrap();
        Lit::Null(lexeme.span().clone())
      }
      _ => return Err(self.build_error::<0>(None)),
    };
    Ok(lit)
  }

  fn pat(&mut self) -> Result<Pat, CompileError> {
    let expr = self.expr()?;
    let span = expr.span().clone();
    Ok(
      Pat::from_expr(expr)
        .ok_or(CompileError::Verify(VerifyError::invalid_case_pat(span)))?,
    )
  }

  fn build_error<const N: usize>(
    &mut self,
    expected: Option<[Token; N]>,
  ) -> CompileError {
    let lexeme = self.advance().unwrap();
    CompileError::Parse(ParseError::tokens(
      lexeme.span().clone(),
      lexeme.token(),
      expected,
    ))
  }

  fn peek(&mut self) -> Result<Lexeme, CompileError> {
    if let Some(current) = &self.current {
      return Ok(current.clone());
    }

    loop {
      let lexeme = self.lexer.next()?;
      if let Token::Comment = lexeme.token() {
      } else {
        self.current = Some(lexeme);
        return self.peek();
      }
    }
  }

  fn advance(&mut self) -> Result<Lexeme, CompileError> {
    self.peek()?;
    Ok(self.current.take().expect("`self.peek` was just called"))
  }

  fn expect<const N: usize>(
    &mut self,
    expected: [Token; N],
  ) -> Result<Lexeme, CompileError> {
    let lexeme = self.advance()?;

    if expected.contains(&lexeme.token()) {
      Ok(lexeme)
    } else {
      Err(CompileError::Parse(ParseError::tokens(
        lexeme.span().clone(),
        lexeme.token(),
        Some(expected),
      )))
    }
  }
}

fn prefix_binding_power(token: Token) -> Option<u8> {
  let power = match token {
    Token::Dash | Token::Bang => 15,
    _ => return None,
  };
  Some(power)
}

fn infix_binding_power(token: Token) -> Option<(u8, u8)> {
  let power = match token {
    Token::Equal => (1, 2),
    Token::And | Token::Or => (3, 4),
    Token::EqualEqual | Token::BangEqual => (5, 6),
    Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual => {
      (7, 8)
    }
    Token::Plus | Token::Dash => (9, 10),
    Token::Star | Token::Slash => (11, 12),
    Token::Dot => (13, 14),
    _ => return None,
  };
  Some(power)
}

fn postfix_binding_power(token: Token) -> Option<u8> {
  let power = match token {
    Token::OpenParen | Token::OpenBracket => 16,
    _ => return None,
  };
  Some(power)
}
