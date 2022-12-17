use crate::{chunk::Chunk, debug::disassemble_chunk, opcode::*, value::Value};

use self::scan::{Scanner, Token, TokenType};

mod scan;

const PREC_NONE: u8 = 0;
const PREC_ASSIGNMENT: u8 = 1;
const PREC_OR: u8 = 2;
const PREC_AND: u8 = 3;
const PREC_EQUALITY: u8 = 4;
const PREC_COMPARISON: u8 = 5;
const PREC_TERM: u8 = 6;
const PREC_FACTOR: u8 = 7;
const PREC_UNARY: u8 = 8;
const PREC_CALL: u8 = 9;
const PREC_PRIMARY: u8 = 10;

type ParseFn = for<'r> fn(&'r mut Compiler<'_>, bool);

struct ParseRule {
  prefix: Option<ParseFn>,
  infix: Option<ParseFn>,
  precedence: u8,
}

impl ParseRule {
  fn new(
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: u8,
  ) -> ParseRule {
    ParseRule {
      prefix,
      infix,
      precedence,
    }
  }

  fn for_token_type(operator_type: TokenType) -> ParseRule {
    match operator_type {
      TokenType::LeftParen => {
        Self::new(Some(|c, a| c.grouping(a)), None, PREC_NONE)
      }
      TokenType::RightParen => Self::new(None, None, PREC_NONE),
      TokenType::LeftBrace => Self::new(None, None, PREC_NONE),
      TokenType::RightBrace => Self::new(None, None, PREC_NONE),
      TokenType::Comma => Self::new(None, None, PREC_NONE),
      TokenType::Dot => Self::new(None, None, PREC_NONE),
      TokenType::Minus => {
        Self::new(Some(|c, a| c.unary(a)), Some(|c, a| c.binary(a)), PREC_TERM)
      }
      TokenType::Plus => Self::new(None, Some(|c, a| c.binary(a)), PREC_TERM),
      TokenType::Semicolon => Self::new(None, None, PREC_NONE),
      TokenType::Slash => {
        Self::new(None, Some(|c, a| c.binary(a)), PREC_FACTOR)
      }
      TokenType::Star => Self::new(None, Some(|c, a| c.binary(a)), PREC_FACTOR),
      TokenType::Bang => Self::new(Some(|c, a| c.unary(a)), None, PREC_NONE),
      TokenType::BangEqual => {
        Self::new(None, Some(|c, a| c.binary(a)), PREC_NONE)
      }
      TokenType::Equal => Self::new(None, None, PREC_NONE),
      TokenType::EqualEqual => {
        Self::new(None, Some(|c, a| c.binary(a)), PREC_EQUALITY)
      }
      TokenType::Greater => {
        Self::new(None, Some(|c, a| c.binary(a)), PREC_COMPARISON)
      }
      TokenType::GreaterEqual => {
        Self::new(None, Some(|c, a| c.binary(a)), PREC_COMPARISON)
      }
      TokenType::Less => {
        Self::new(None, Some(|c, a| c.binary(a)), PREC_COMPARISON)
      }
      TokenType::LessEqual => {
        Self::new(None, Some(|c, a| c.binary(a)), PREC_COMPARISON)
      }
      TokenType::Identifier => {
        Self::new(Some(|c, a| c.variable(a)), None, PREC_NONE)
      }
      TokenType::String => Self::new(Some(|c, a| c.string(a)), None, PREC_NONE),
      TokenType::Number => Self::new(Some(|c, a| c.number(a)), None, PREC_NONE),
      TokenType::And => Self::new(None, None, PREC_NONE),
      TokenType::Class => Self::new(None, None, PREC_NONE),
      TokenType::Else => Self::new(None, None, PREC_NONE),
      TokenType::False => Self::new(Some(|c, a| c.literal(a)), None, PREC_NONE),
      TokenType::For => Self::new(None, None, PREC_NONE),
      TokenType::Fun => Self::new(None, None, PREC_NONE),
      TokenType::If => Self::new(None, None, PREC_NONE),
      TokenType::Nil => Self::new(Some(|c, a| c.literal(a)), None, PREC_NONE),
      TokenType::Or => Self::new(None, None, PREC_NONE),
      TokenType::Print => Self::new(None, None, PREC_NONE),
      TokenType::Return => Self::new(None, None, PREC_NONE),
      TokenType::Super => Self::new(None, None, PREC_NONE),
      TokenType::This => Self::new(None, None, PREC_NONE),
      TokenType::True => Self::new(Some(|c, a| c.literal(a)), None, PREC_NONE),
      TokenType::Var => Self::new(None, None, PREC_NONE),
      TokenType::While => Self::new(None, None, PREC_NONE),
      TokenType::Error => Self::new(None, None, PREC_NONE),
      TokenType::Eof => Self::new(None, None, PREC_NONE),
    }
  }

  fn precedence(&self) -> u8 {
    self.precedence
  }
}

pub struct Compiler<'a> {
  scanner: Scanner,
  previous: Option<Token>,
  current: Option<Token>,
  current_chunk: Option<&'a mut Chunk>,
  had_error: bool,
  panic_mode: bool,
}

impl<'a> Compiler<'a> {
  pub fn new(source: &str) -> Compiler<'a> {
    Compiler {
      scanner: Scanner::new(source),
      previous: None,
      current: None,
      current_chunk: None,
      had_error: false,
      panic_mode: false,
    }
  }

  pub fn compile(&mut self, chunk: &'a mut Chunk) -> bool {
    self.current_chunk = Some(chunk);
    self.advance();

    while !self.expect(TokenType::Eof) {
      self.declaration();
    }

    self.end();
    !self.had_error
  }

  fn declaration(&mut self) {
    if self.expect(TokenType::Var) {
      self.var_declaration();
    } else {
      self.statement();
    }

    if self.panic_mode {
      self.synchronize();
    }
  }

  fn var_declaration(&mut self) {
    let global = self.parse_variable("expected variable name");

    if self.expect(TokenType::Equal) {
      self.expression();
    } else {
      self.emit_byte(OP_NIL);
    }

    self.consume(
      TokenType::Semicolon,
      "expect ';' after variable declaration",
    );

    self.define_variable(global);
  }

  fn synchronize(&mut self) {
    self.panic_mode = false;

    loop {
      if self.current().ty == TokenType::Eof {
        return;
      } else if self.previous().ty == TokenType::Semicolon {
        return;
      }

      match self.current().ty {
        TokenType::Class
        | TokenType::Fun
        | TokenType::Var
        | TokenType::For
        | TokenType::If
        | TokenType::While
        | TokenType::Print
        | TokenType::Return => return,
        _ => {}
      }

      self.advance();
    }
  }

  fn statement(&mut self) {
    if self.expect(TokenType::Print) {
      self.print_statement();
    } else {
      self.expression_statement();
    }
  }

  fn print_statement(&mut self) {
    self.expression();
    self.consume(TokenType::Semicolon, "expect ';' after value");
    self.emit_byte(OP_PRINT);
  }

  fn expression_statement(&mut self) {
    self.expression();
    self.consume(TokenType::Semicolon, "expect ';' after value");
    self.emit_byte(OP_POP);
  }

  fn expression(&mut self) {
    self.parse_precedence(PREC_ASSIGNMENT);
  }

  fn number(&mut self, _can_assign: bool) {
    let value = Value::Number(
      self
        .previous()
        .as_str()
        .parse()
        .expect("failed to parse number token"),
    );
    self.emit_constant(value);
  }

  fn string(&mut self, _can_assign: bool) {
    let len = self.previous().as_str().len();
    self
      .emit_constant(Value::from_string(&self.previous().as_str()[1..len - 1]));
  }

  fn variable(&mut self, can_assign: bool) {
    self.named_variable(self.previous(), can_assign);
  }

  fn named_variable(&mut self, token: Token, can_assign: bool) {
    let name = self.identifier_constant(token);

    if can_assign && self.expect(TokenType::Equal) {
      self.expression();
      self.emit_bytes(OP_SET_GLOBAL, name);
    } else {
      self.emit_bytes(OP_GET_GLOBAL, name);
    }
  }

  fn grouping(&mut self, _can_assign: bool) {
    self.expression();
    self.consume(TokenType::RightParen, "expected ')' after expression.");
  }

  fn unary(&mut self, _can_assign: bool) {
    let operator_type = self.previous().ty;

    self.parse_precedence(PREC_UNARY);

    match operator_type {
      TokenType::Minus => self.emit_byte(OP_NEGATE),
      TokenType::Bang => self.emit_byte(OP_NOT),
      _ => {}
    }
  }

  fn binary(&mut self, _can_assign: bool) {
    let operator_type = self.previous().ty;

    let rule = ParseRule::for_token_type(operator_type.clone());
    self.parse_precedence(rule.precedence() + 1);

    match operator_type {
      TokenType::BangEqual => self.emit_bytes(OP_EQUAL, OP_NOT),
      TokenType::EqualEqual => self.emit_byte(OP_EQUAL),
      TokenType::GreaterEqual => self.emit_bytes(OP_LESS, OP_NOT),
      TokenType::Greater => self.emit_byte(OP_GREATER),
      TokenType::LessEqual => self.emit_bytes(OP_GREATER, OP_NOT),
      TokenType::Less => self.emit_byte(OP_LESS),
      TokenType::Plus => self.emit_byte(OP_ADD),
      TokenType::Minus => self.emit_byte(OP_SUBTRACT),
      TokenType::Star => self.emit_byte(OP_MULTIPLY),
      TokenType::Slash => self.emit_byte(OP_DIVIDE),
      _ => {}
    }
  }

  fn literal(&mut self, _can_assign: bool) {
    match self.previous().ty {
      TokenType::False => self.emit_byte(OP_FALSE),
      TokenType::Nil => self.emit_byte(OP_NIL),
      TokenType::True => self.emit_byte(OP_TRUE),
      _ => panic!("unexpected literal token"),
    }
  }

  fn parse_precedence(&mut self, precedence: u8) {
    self.advance();

    let prefix = match ParseRule::for_token_type(self.previous().ty).prefix {
      Some(prefix) => prefix,
      None => {
        self.error("expected expression");
        return;
      }
    };

    let can_assign = precedence <= PREC_ASSIGNMENT;
    prefix(self, can_assign);

    while precedence
      <= ParseRule::for_token_type(self.current().ty).precedence()
    {
      self.advance();
      let infix = ParseRule::for_token_type(self.previous().ty)
        .infix
        .expect("no infix found for parse rule");
      infix(self, can_assign);
    }

    if can_assign && self.expect(TokenType::Equal) {
      self.error("invalid assignment target");
    }
  }

  fn parse_variable(&mut self, msg: &str) -> u8 {
    self.consume(TokenType::Identifier, msg);
    self.identifier_constant(self.previous().clone())
  }

  fn identifier_constant(&mut self, name: Token) -> u8 {
    self.make_constant(Value::from_string(name.as_str()))
  }

  fn define_variable(&mut self, global: u8) {
    self.emit_bytes(OP_DEFINE_GLOBAL, global);
  }

  fn advance(&mut self) {
    self.previous = self.current.take();

    loop {
      self.current = Some(self.scanner.scan_token());
      if self.current().ty != TokenType::Error {
        break;
      }
      self.error_at_current(self.current().as_str());
    }
  }

  fn consume(&mut self, ty: TokenType, msg: &str) {
    if self.current().ty == ty {
      self.advance();
      return;
    }
    self.error_at_current(msg)
  }

  // This was `match` in clox but `match` is a Rust keyword.
  fn expect(&mut self, ty: TokenType) -> bool {
    let check = self.check(ty);
    if check {
      self.advance();
    }
    check
  }

  fn check(&self, ty: TokenType) -> bool {
    self.current().ty == ty
  }

  fn error_at_current(&mut self, msg: &str) {
    self.error_at(self.current(), msg)
  }

  fn error(&mut self, msg: &str) {
    self.error_at(self.previous(), msg)
  }

  fn error_at(&mut self, token: Token, msg: &str) {
    if self.panic_mode {
      return;
    }

    self.panic_mode = true;

    eprint!("[line {}] Error", token.line);
    match token.ty {
      TokenType::Eof => eprint!(" at end"),
      TokenType::Error => {}
      _ => eprint!(" at {}", token.as_str()),
    }
    eprintln!(": {}", msg);

    self.had_error = true;
  }

  fn end(&mut self) {
    self.emit_return();
    if !self.had_error {
      disassemble_chunk(self.current_chunk(), "code");
    }
  }

  fn emit_return(&mut self) {
    self.emit_byte(OP_RETURN);
  }

  fn emit_constant(&mut self, value: Value) {
    let constant = self.make_constant(value);
    self.emit_bytes(OP_CONSTANT, constant);
  }

  fn make_constant(&mut self, value: Value) -> u8 {
    self.current_chunk().add_constant(value)
  }

  fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
    self.emit_byte(byte1);
    self.emit_byte(byte2);
  }

  fn emit_byte(&mut self, byte: u8) {
    let line = self.previous().line;
    self.current_chunk().write(byte, line);
  }

  fn current_chunk(&mut self) -> &mut Chunk {
    self.current_chunk.as_mut().expect("chunk not set")
  }

  fn current(&self) -> Token {
    self.current.as_ref().cloned().expect("current not set")
  }

  fn previous(&self) -> Token {
    self.previous.as_ref().cloned().expect("preivous not set")
  }
}
