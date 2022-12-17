use std::{mem, rc::Rc};

use gc::{Gc, GcCell};

use crate::{
  chunk::{Chunk, Function, Op},
  error::{Error, Reason},
  opcode::Opcode,
  registry::NativeLambdaRegistry,
  value::{Lambda, Tagged, Upvalue, Value},
};

use self::{frame::Frame, stack::Stack};

mod frame;
mod stack;

macro_rules! binary {
  ($self:expr, $op:ident) => {{
    let right = $self.stack_pop()?;
    let left = $self.stack_pop()?;
    let result = left
      .$op(&right)
      .ok_or_else(|| $self.build_error(Reason::Type))?;
    $self.stack.push(result);
    Ok(())
  }};
}

macro_rules! unary {
  ($self:expr, $op:ident) => {{
    let value = $self.stack_pop()?;
    let result = value.$op().ok_or_else(|| $self.build_error(Reason::Type))?;
    $self.stack.push(result);
    Ok(())
  }};
}

pub struct Fiber {
  registry: Rc<NativeLambdaRegistry>,
  chunk: Rc<Chunk>,
  ip: usize,
  bp: usize,
  stack: Stack,
}

impl Fiber {
  pub fn new(registry: Rc<NativeLambdaRegistry>, function: Function) -> Fiber {
    let mut stack = Stack::new();

    stack.push(Value::Lambda(Gc::new(Lambda::new(
      function.arity as usize,
      Rc::clone(&function.chunk),
      function.locals as usize,
      Vec::new(),
    ))));

    for _ in 0..function.locals {
      stack.push(Value::Null);
    }

    Fiber {
      registry,
      chunk: Rc::clone(&function.chunk),
      ip: 0,
      bp: 0,
      stack,
    }
  }

  pub fn step(&mut self) -> Result<Status, Error> {
    let op = self.next_op()?;
    let opcode = op
      .opcode()
      .ok_or_else(|| self.build_error(Reason::InvalidOpcode(op.code())))?;
    match opcode {
      // Values
      Opcode::Nul => self.nul(),
      Opcode::Tru => self.tru(),
      Opcode::Fls => self.fls(),
      Opcode::Flt => self.flt(op.operand()),
      Opcode::Int => self.int(op.operand()),
      Opcode::Str => self.str(op.operand() as usize),
      Opcode::Arr => self.arr(),
      Opcode::Map => self.map(),
      Opcode::Lmd => self.lmd(op.operand() as usize),
      Opcode::Nal => self.nal(op.operand() as usize),
      Opcode::Pop => self.pop(),
      // Locals and upvalues
      Opcode::Lod => self.lod(op.operand() as usize),
      Opcode::Sav => self.sav(op.operand() as usize),
      Opcode::Lou => self.lou(op.operand() as usize),
      Opcode::Sau => self.sau(op.operand() as usize),
      Opcode::Clu => self.clu(),
      // Code navigation
      Opcode::Jmp => self.jmp(op.operand() as usize),
      Opcode::Jit => self.jit(op.operand() as usize),
      Opcode::Jif => self.jif(op.operand() as usize),
      // Stack manipulation
      Opcode::Dup => self.dup(),
      Opcode::Swp => self.swp(),
      Opcode::Rot => self.rot(),
      // Primitive operations
      Opcode::Add => self.add(),
      Opcode::Sub => self.sub(),
      Opcode::Mul => self.mul(),
      Opcode::Div => self.div(),
      Opcode::Rem => self.rem(),
      Opcode::Neg => self.neg(),
      Opcode::Eql => self.eql(),
      Opcode::Neq => self.neq(),
      Opcode::Gtn => self.gtn(),
      Opcode::Gte => self.gte(),
      Opcode::Ltn => self.ltn(),
      Opcode::Lte => self.lte(),
      Opcode::Not => self.not(),
      // Map/array operations
      Opcode::Get => self.get(),
      Opcode::Set => self.set(),
      Opcode::Apn => self.apn(),
      // Tagged operations
      Opcode::Tag => self.tag(),
      Opcode::Utg => self.utg(),
      Opcode::Gtg => self.gtg(),
      // Lambda operations
      Opcode::Cal => self.cal(op.operand() as usize),
      Opcode::Ret => return self.ret(),
    }?;
    Ok(Status::Running)
  }

  fn nul(&mut self) -> Result<(), Error> {
    self.stack.push(Value::Null);
    Ok(())
  }

  fn tru(&mut self) -> Result<(), Error> {
    self.stack.push(Value::Bool(true));
    Ok(())
  }

  fn fls(&mut self) -> Result<(), Error> {
    self.stack.push(Value::Bool(false));
    Ok(())
  }

  fn flt(&mut self, bits: u64) -> Result<(), Error> {
    let float = Value::Float(f64::from_bits(bits));
    self.stack.push(float);
    Ok(())
  }

  fn int(&mut self, bits: u64) -> Result<(), Error> {
    let int = Value::Int(bits as i64);
    self.stack.push(int);
    Ok(())
  }

  fn str(&mut self, index: usize) -> Result<(), Error> {
    let string = self
      .chunk
      .constant(index as usize)
      .ok_or_else(|| self.build_error(Reason::InvalidData(index)))?
      .as_str()
      .ok_or_else(|| self.build_error(Reason::WrongConstantType))?
      // TODO: Don't copy string on each use
      .to_string();

    self.stack.push(Value::String(Gc::new(string)));

    Ok(())
  }

  fn arr(&mut self) -> Result<(), Error> {
    let array = Value::array();
    self.stack.push(array);
    Ok(())
  }

  fn map(&mut self) -> Result<(), Error> {
    let map = Value::map();
    self.stack.push(map);
    Ok(())
  }

  fn lmd(&mut self, index: usize) -> Result<(), Error> {
    let function = self
      .chunk
      .constant(index as usize)
      .ok_or_else(|| self.build_error(Reason::InvalidData(index)))?
      .as_function()
      .cloned()
      .ok_or_else(|| self.build_error(Reason::WrongConstantType))?;

    let upvalues = function
      .upvalues
      .iter()
      .map(|(index, is_local)| self.capture_upvalue(*index as usize, *is_local))
      .collect::<Result<_, Error>>()?;

    let lambda = Lambda::new(
      function.arity as usize,
      Rc::clone(&function.chunk),
      function.locals as usize,
      upvalues,
    );

    self.stack.push(Value::Lambda(Gc::new(lambda)));
    Ok(())
  }

  fn nal(&mut self, id: usize) -> Result<(), Error> {
    let native_lambda = self
      .registry
      .get(id)
      .cloned()
      .ok_or_else(|| self.build_error(Reason::InvalidNativeLambda(id)))?;

    self.stack.push(Value::NativeLambda(native_lambda));

    Ok(())
  }

  fn capture_upvalue(
    &mut self,
    index: usize,
    is_local: bool,
  ) -> Result<Gc<GcCell<Upvalue>>, Error> {
    if is_local {
      if self.bp + index < self.stack.len() {
        Ok(self.stack.upvalue(self.bp + index))
      } else {
        Err(self.build_error(Reason::InvalidUpvalue(index)))
      }
    } else {
      let lambda = self
        .stack_get(self.bp)?
        .as_lambda()
        .ok_or_else(|| self.build_error(Reason::WrongConstantType))?;
      let upvalue = lambda
        .upvalue(index)
        .ok_or_else(|| self.build_error(Reason::InvalidUpvalue(index)))?;
      Ok(Gc::clone(upvalue))
    }
  }

  fn pop(&mut self) -> Result<(), Error> {
    self.stack_pop()?;
    Ok(())
  }

  fn lod(&mut self, local: usize) -> Result<(), Error> {
    let value = self.stack_get(self.bp + local)?.clone();
    self.stack.push(value);
    Ok(())
  }

  fn sav(&mut self, local: usize) -> Result<(), Error> {
    let value = self.stack_last()?.clone();
    self.stack_set(self.bp + local, value)?;
    Ok(())
  }

  fn lou(&mut self, index: usize) -> Result<(), Error> {
    let lambda = self
      .stack_get(self.bp)?
      .as_lambda()
      .ok_or_else(|| self.build_error(Reason::WrongConstantType))?;

    let upvalue = lambda
      .upvalue(index)
      .ok_or_else(|| self.build_error(Reason::InvalidUpvalue(index)))?;

    let value = match &*upvalue.borrow() {
      Upvalue::Open(index) => self.stack_get(*index)?.clone(),
      Upvalue::Closed(value) => value.clone(),
    };

    self.stack.push(value);
    Ok(())
  }

  fn sau(&mut self, index: usize) -> Result<(), Error> {
    let lambda = self
      .stack_get(self.bp)?
      .as_lambda()
      .ok_or_else(|| self.build_error(Reason::WrongConstantType))?;

    let upvalue = lambda
      .upvalue(index)
      .cloned()
      .ok_or_else(|| self.build_error(Reason::InvalidUpvalue(index)))?;

    let value = self.stack_last()?.clone();

    let mut upvalue_mut = upvalue.borrow_mut();
    match &mut *upvalue_mut {
      Upvalue::Open(index) => self.stack_set(*index, value)?,
      Upvalue::Closed(upvalue_value) => *upvalue_value = value,
    };

    Ok(())
  }

  fn clu(&mut self) -> Result<(), Error> {
    self
      .stack
      .close_upvalue()
      .ok_or_else(|| self.build_error(Reason::EmptyStack))
  }

  fn jmp(&mut self, offset: usize) -> Result<(), Error> {
    self.ip = offset;
    Ok(())
  }

  fn jit(&mut self, offset: usize) -> Result<(), Error> {
    if let Value::Bool(true) = self.stack_pop()? {
      self.ip = offset;
    }
    Ok(())
  }

  fn jif(&mut self, offset: usize) -> Result<(), Error> {
    if let Value::Bool(false) = self.stack_pop()? {
      self.ip = offset;
    }
    Ok(())
  }

  fn dup(&mut self) -> Result<(), Error> {
    let value = self.stack_last()?.clone();
    self.stack.push(value);

    Ok(())
  }

  fn swp(&mut self) -> Result<(), Error> {
    let second = self.stack_pop()?;
    let first = self.stack_pop()?;

    self.stack.push(second);
    self.stack.push(first);

    Ok(())
  }

  fn rot(&mut self) -> Result<(), Error> {
    let third = self.stack_pop()?;
    let second = self.stack_pop()?;
    let first = self.stack_pop()?;

    self.stack.push(third);
    self.stack.push(first);
    self.stack.push(second);

    Ok(())
  }

  fn add(&mut self) -> Result<(), Error> {
    binary!(self, add)
  }

  fn sub(&mut self) -> Result<(), Error> {
    binary!(self, sub)
  }

  fn mul(&mut self) -> Result<(), Error> {
    binary!(self, mul)
  }

  fn div(&mut self) -> Result<(), Error> {
    binary!(self, div)
  }

  fn rem(&mut self) -> Result<(), Error> {
    binary!(self, rem)
  }

  fn neg(&mut self) -> Result<(), Error> {
    unary!(self, neg)
  }

  fn eql(&mut self) -> Result<(), Error> {
    let right = self.stack_pop()?;
    let left = self.stack_pop()?;
    let result = left.eql(&right);
    self.stack.push(result);
    Ok(())
  }

  fn neq(&mut self) -> Result<(), Error> {
    let right = self.stack_pop()?;
    let left = self.stack_pop()?;
    let result = left.neq(&right);
    self.stack.push(result);
    Ok(())
  }

  fn gtn(&mut self) -> Result<(), Error> {
    binary!(self, gtn)
  }

  fn gte(&mut self) -> Result<(), Error> {
    binary!(self, gte)
  }

  fn ltn(&mut self) -> Result<(), Error> {
    binary!(self, ltn)
  }

  fn lte(&mut self) -> Result<(), Error> {
    binary!(self, lte)
  }

  fn not(&mut self) -> Result<(), Error> {
    unary!(self, not)
  }

  fn get(&mut self) -> Result<(), Error> {
    let key = self.stack_pop()?;
    let receiver = self.stack_pop()?;

    let value = receiver
      .get(&key)
      .ok_or_else(|| self.build_error(Reason::InvalidKey(key)))?;

    self.stack.push(value);

    Ok(())
  }

  fn set(&mut self) -> Result<(), Error> {
    let key = self.stack_pop()?;
    let value = self.stack_pop()?;
    let receiver = self.stack_pop()?;

    receiver
      .set(key.clone(), value.clone())
      .ok_or_else(|| self.build_error(Reason::InvalidKey(key)))?;

    self.stack.push(value);

    Ok(())
  }

  fn apn(&mut self) -> Result<(), Error> {
    let value = self.stack_pop()?;

    let array_value = self.stack_pop()?;
    let array = array_value
      .as_array()
      .ok_or_else(|| self.build_error(Reason::Type))?;

    array.borrow_mut().push(&value);

    self.stack.push(value);

    Ok(())
  }

  fn tag(&mut self) -> Result<(), Error> {
    let tag_value = self.stack_pop()?;
    let tag = tag_value
      .as_string()
      .ok_or_else(|| self.build_error(Reason::InvalidTag(tag_value.clone())))?;

    let value = self.stack_pop()?;

    self
      .stack
      .push(Value::Tagged(Gc::new(Tagged::new(Gc::clone(tag), value))));

    Ok(())
  }

  fn utg(&mut self) -> Result<(), Error> {
    let value = self.stack_pop()?;
    let tagged = value
      .as_tagged()
      .ok_or_else(|| self.build_error(Reason::Type))?;
    self.stack.push(tagged.value().clone());
    Ok(())
  }

  fn gtg(&mut self) -> Result<(), Error> {
    let value = self.stack_pop()?;
    let tag = value
      .as_tagged()
      .map(|tagged| Value::String(Gc::clone(tagged.tag())))
      .unwrap_or(Value::Null);
    self.stack.push(tag);
    Ok(())
  }

  fn cal(&mut self, arity: usize) -> Result<(), Error> {
    let bp = self.stack.len() - arity - 1;

    let value = self.stack_get(bp)?.clone();
    match &value {
      Value::Lambda(lambda) => {
        if arity != lambda.arity() {
          return Err(self.build_error(Reason::WrongArity));
        }

        let chunk = Rc::clone(lambda.chunk());

        let frame = Frame {
          chunk: mem::replace(&mut self.chunk, chunk),
          ip: mem::replace(&mut self.ip, 0),
          bp: mem::replace(&mut self.bp, bp),
        };
        self.stack.push_frame(frame);

        for _ in 0..lambda.locals() {
          self.stack.push(Value::Null);
        }
      }
      Value::NativeLambda(native_lambda) => {
        if arity != 1 {
          return Err(self.build_error(Reason::WrongArity));
        }

        let argument = self.stack_pop()?;
        let result = native_lambda.call(argument);
        self.stack_pop()?; // pop the native lambda off as well.

        self.stack.push(result);
      }
      _ => return Err(self.build_error(Reason::ValueNotCallable(value.clone()))),
    }

    Ok(())
  }

  fn ret(&mut self) -> Result<Status, Error> {
    if self.stack.is_frames_empty() {
      Ok(Status::Done)
    } else {
      // The return value will be at the top of the stack when a lambda completes its
      // executation. We save it temporarily and pop the rest of the stack up to the
      // base pointer.
      let return_value = self.stack_pop()?;
      while self.stack.len() > self.bp {
        // TODO: Close multiple upvalues at the same time.
        self
          .stack
          .close_upvalue()
          .ok_or_else(|| self.build_error(Reason::EmptyStack))?;
      }
      self.stack.push(return_value);

      // We then restore the values from the previous frame.
      let frame = self
        .stack
        .pop_frame()
        .ok_or_else(|| self.build_error(Reason::EmptyFrameStack))?;
      self.chunk = frame.chunk;
      self.ip = frame.ip;
      self.bp = frame.bp;

      Ok(Status::Running)
    }
  }

  fn stack_pop(&mut self) -> Result<Value, Error> {
    self
      .stack
      .pop()
      .ok_or_else(|| self.build_error(Reason::EmptyStack))
  }

  fn stack_last(&self) -> Result<&Value, Error> {
    self
      .stack
      .last()
      .ok_or_else(|| self.build_error(Reason::EmptyStack))
  }

  fn stack_get(&self, local: usize) -> Result<&Value, Error> {
    self
      .stack
      .get(local)
      .ok_or_else(|| self.build_error(Reason::EmptyStack))
  }

  fn stack_set(&mut self, local: usize, value: Value) -> Result<(), Error> {
    self
      .stack
      .set(local, value)
      .ok_or_else(|| self.build_error(Reason::EmptyStack))
  }

  fn build_error(&self, reason: Reason) -> Error {
    let mut trace = self.stack.build_trace();
    // TODO: `self.ip` doesn't reflect current op, but next op, so `- 1` is required to get current op.
    trace.push(self.chunk.info().and_then(|info| info.span(self.ip - 1)));
    Error::new(reason, trace)
  }

  fn next_op(&mut self) -> Result<Op, Error> {
    let op = self
      .chunk
      .op(self.ip)
      // TODO: `self.ip` doesn't reflect current op, but next op.
      .ok_or_else(|| self.build_error(Reason::InvalidCode(self.ip)))?;
    self.ip += 1 + op.size() as usize;
    Ok(op)
  }
}

pub enum Status {
  Done,
  Running,
  Yield,
}
