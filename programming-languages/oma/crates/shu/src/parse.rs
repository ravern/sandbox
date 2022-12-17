use std::rc::Rc;

use crate::chunk::{Chunk, Constant, Function};

const SECTION_INFO: u8 = 1;
const SECTION_DATA: u8 = 2;
const SECTION_CODE: u8 = 3;

const CONSTANT_STR: u8 = 1;
const CONSTANT_FUN: u8 = 2;

pub struct Parser<'a> {
  bytes: &'a [u8],
  current: usize,
}

impl<'a> Parser<'a> {
  pub fn new(bytes: &'a [u8]) -> Parser<'a> {
    Parser { bytes, current: 0 }
  }

  pub fn parse(mut self) -> Result<Function, ParseError> {
    self.expect([b'O', b'M', b'A', 1])?;

    self.function()
  }

  fn chunk(&mut self) -> Result<Chunk, ParseError> {
    let info = if self.peek()? == [SECTION_INFO] {
      unimplemented!();
    } else {
      None
    };

    let data = self.data()?;
    let code = self.code()?;

    Ok(Chunk { info, data, code })
  }

  fn data(&mut self) -> Result<Box<[Constant]>, ParseError> {
    self.expect([SECTION_DATA])?;

    let len = u64::from_le_bytes(self.advance::<8>()?) as usize;

    (0..len)
      .map(|_| self.constant())
      .collect::<Result<Box<[Constant]>, ParseError>>()
  }

  fn code(&mut self) -> Result<Box<[u8]>, ParseError> {
    self.expect([SECTION_CODE])?;

    let len = u64::from_le_bytes(self.advance::<8>()?) as usize;

    let code = self
      .bytes
      .get(self.current..self.current + len)
      .ok_or(ParseError {})?
      .into();
    self.current += len;

    Ok(code)
  }

  fn constant(&mut self) -> Result<Constant, ParseError> {
    match self.peek::<1>()? {
      [CONSTANT_STR] => self.constant_string().map(Constant::String),
      [CONSTANT_FUN] => self.constant_function().map(Constant::Function),
      _ => Err(ParseError {}),
    }
  }

  fn constant_string(&mut self) -> Result<String, ParseError> {
    self.expect([CONSTANT_STR])?;

    let len = u64::from_le_bytes(self.advance::<8>()?) as usize;

    let bytes = self
      .bytes
      .get(self.current..self.current + len)
      .ok_or(ParseError {})?
      .to_vec();
    self.current += len;

    String::from_utf8(bytes).map_err(|_| ParseError {})
  }

  fn constant_function(&mut self) -> Result<Function, ParseError> {
    self.expect([CONSTANT_FUN])?;

    self.function()
  }

  fn function(&mut self) -> Result<Function, ParseError> {
    let arity = u64::from_le_bytes(self.advance::<8>()?);

    let chunk = self.chunk()?;

    let locals = u64::from_le_bytes(self.advance::<8>()?);

    let upvalues_len = u64::from_le_bytes(self.advance::<8>()?) as usize;

    let upvalues = (0..upvalues_len)
      .map(|_| {
        Ok((
          u64::from_le_bytes(self.advance::<8>()?),
          match self.advance::<1>()? {
            [0] => false,
            [1] => true,
            _ => return Err(ParseError {}),
          },
        ))
      })
      .collect::<Result<Vec<(u64, bool)>, ParseError>>()?;

    Ok(Function {
      arity,
      chunk: Rc::new(chunk),
      locals,
      upvalues,
    })
  }

  fn expect<const N: usize>(&mut self, expected: [u8; N]) -> Result<[u8; N], ParseError> {
    let mut bytes = [0u8; N];
    bytes.clone_from_slice(
      self
        .bytes
        .get(self.current..self.current + N)
        .ok_or(ParseError {})?,
    );
    self.current += N;
    if bytes == expected {
      Ok(bytes)
    } else {
      Err(ParseError {})
    }
  }

  fn advance<const N: usize>(&mut self) -> Result<[u8; N], ParseError> {
    let bytes = self.peek::<N>()?;
    self.current += N;
    Ok(bytes)
  }

  fn peek<const N: usize>(&mut self) -> Result<[u8; N], ParseError> {
    let mut bytes = [0u8; N];
    bytes.clone_from_slice(
      self
        .bytes
        .get(self.current..self.current + N)
        .ok_or(ParseError {})?,
    );
    Ok(bytes)
  }
}

#[derive(Debug)]
pub struct ParseError {}
