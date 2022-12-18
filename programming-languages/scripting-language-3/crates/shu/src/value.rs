use std::{
  cmp::PartialOrd,
  collections::HashMap,
  fmt,
  ops::{Add, Div, Mul, Rem, Sub},
  rc::Rc,
};

use gc::{unsafe_empty_trace, Finalize, Gc, GcCell, Trace};

use crate::chunk::Chunk;

macro_rules! arithmetic {
  ($left:expr, $right:expr, $op:ident) => {
    match ($left, $right) {
      (Value::Int(left), Value::Int(right)) => Some(Value::Int(left.$op(right))),
      (Value::Int(left), Value::Float(right)) => Some(Value::Float((*left as f64).$op(right))),
      (Value::Float(left), Value::Int(right)) => Some(Value::Float(left.$op(*right as f64))),
      (Value::Float(left), Value::Float(right)) => Some(Value::Float(left.$op(right))),
      _ => None,
    }
  };
}

macro_rules! comparison {
  ($left:expr, $right:expr, $op:ident) => {
    match ($left, $right) {
      (Value::Int(left), Value::Int(right)) => Some(Value::Bool(left.$op(right))),
      (Value::Int(left), Value::Float(right)) => Some(Value::Bool((*left as f64).$op(right))),
      (Value::Float(left), Value::Int(right)) => Some(Value::Bool(left.$op(&(*right as f64)))),
      (Value::Float(left), Value::Float(right)) => Some(Value::Bool(left.$op(right))),
      _ => None,
    }
  };
}

#[derive(Clone, Debug, Finalize, Trace)]
pub enum Value {
  Null,
  Bool(bool),
  Int(i64),
  Float(f64),
  String(Gc<String>),
  Array(Gc<GcCell<Array>>),
  Map(Gc<GcCell<Map>>),
  Tagged(Gc<Tagged>),
  Lambda(Gc<Lambda>),
  NativeLambda(Gc<NativeLambda>),
}

impl Value {
  pub fn array() -> Value {
    Value::Array(Gc::new(GcCell::new(Array::new())))
  }

  pub fn map() -> Value {
    Value::Map(Gc::new(GcCell::new(Map::new())))
  }

  pub fn add(&self, other: &Value) -> Option<Value> {
    if let (Value::String(left), Value::String(right)) = (self, other) {
      let result = [left.as_str(), right.as_str()].concat();
      Some(Value::String(Gc::new(result)))
    } else {
      arithmetic!(self, other, add)
    }
  }

  pub fn sub(&self, other: &Value) -> Option<Value> {
    arithmetic!(self, other, sub)
  }

  pub fn mul(&self, other: &Value) -> Option<Value> {
    arithmetic!(self, other, mul)
  }

  pub fn div(&self, other: &Value) -> Option<Value> {
    arithmetic!(self, other, div)
  }

  pub fn rem(&self, other: &Value) -> Option<Value> {
    arithmetic!(self, other, rem)
  }

  pub fn neg(&self) -> Option<Value> {
    match self {
      Value::Int(int) => Some(Value::Int(-int)),
      Value::Float(float) => Some(Value::Float(-float)),
      _ => None,
    }
  }

  pub fn eql(&self, other: &Value) -> Value {
    match (self, other) {
      (Value::Null, Value::Null) => Value::Bool(true),
      (Value::Bool(left), Value::Bool(right)) => Value::Bool(left == right),
      (Value::Int(left), Value::Int(right)) => Value::Bool(left == right),
      (Value::Float(left), Value::Float(right)) => Value::Bool(left == right),
      (Value::String(left), Value::String(right)) => Value::Bool(left == right),
      (Value::Array(left), Value::Array(right)) => Value::Bool(Gc::ptr_eq(left, right)),
      (Value::Map(left), Value::Map(right)) => Value::Bool(Gc::ptr_eq(left, right)),
      _ => Value::Bool(false),
    }
  }

  pub fn neq(&self, other: &Value) -> Value {
    if let Value::Bool(result) = self.eql(other) {
      Value::Bool(!result)
    } else {
      unreachable!();
    }
  }

  pub fn gtn(&self, other: &Value) -> Option<Value> {
    comparison!(self, other, gt)
  }

  pub fn gte(&self, other: &Value) -> Option<Value> {
    match self.ltn(other) {
      Some(Value::Bool(result)) => Some(Value::Bool(!result)),
      None => None,
      _ => unreachable!(),
    }
  }

  pub fn ltn(&self, other: &Value) -> Option<Value> {
    comparison!(self, other, lt)
  }

  pub fn lte(&self, other: &Value) -> Option<Value> {
    match self.gtn(other) {
      Some(Value::Bool(result)) => Some(Value::Bool(!result)),
      None => None,
      _ => unreachable!(),
    }
  }

  pub fn not(&self) -> Option<Value> {
    if let Value::Bool(bool) = self {
      Some(Value::Bool(!bool))
    } else {
      None
    }
  }

  pub fn get(&self, key: &Value) -> Option<Value> {
    match self {
      Value::Array(array) => array.borrow().get(key),
      Value::Map(map) => map.borrow().get(key),
      _ => None,
    }
  }

  pub fn set(&self, key: Value, value: Value) -> Option<()> {
    match self {
      Value::Array(array) => array.borrow_mut().set(key, value),
      Value::Map(map) => map.borrow_mut().set(key, value),
      _ => None,
    }
  }

  pub fn as_string(&self) -> Option<&Gc<String>> {
    if let Value::String(string) = self {
      Some(string)
    } else {
      None
    }
  }

  pub fn as_tagged(&self) -> Option<&Tagged> {
    if let Value::Tagged(tagged) = self {
      Some(tagged)
    } else {
      None
    }
  }

  pub fn as_array(&self) -> Option<&Gc<GcCell<Array>>> {
    if let Value::Array(array) = self {
      Some(array)
    } else {
      None
    }
  }

  pub fn as_lambda(&self) -> Option<&Lambda> {
    if let Value::Lambda(lambda) = self {
      Some(lambda)
    } else {
      None
    }
  }
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Value::Null => write!(f, "null"),
      Value::Bool(bool) => write!(f, "{}", bool),
      Value::Int(int) => write!(f, "{}", int),
      Value::Float(float) => write!(f, "{}", float),
      Value::String(string) => write!(f, "{}", string),
      Value::Array(array) => write!(f, "{}", array.borrow()),
      Value::Map(map) => write!(f, "{}", map.borrow()),
      Value::Tagged(tagged) => write!(f, "{}", tagged),
      Value::Lambda(_) => write!(f, "<<lambda>>"),
      Value::NativeLambda(_) => write!(f, "<<native lambda>>"),
    }
  }
}

#[derive(Debug, Finalize, Trace)]
pub struct Array {
  array: Vec<Value>,
}

impl Array {
  pub fn new() -> Array {
    Array { array: Vec::new() }
  }

  pub fn set(&mut self, key: Value, value: Value) -> Option<()> {
    if let Value::Int(key) = key {
      if key < 0 {
        None
      } else {
        *self.array.get_mut(key as usize)? = value;
        Some(())
      }
    } else {
      None
    }
  }

  pub fn get(&self, key: &Value) -> Option<Value> {
    if let Value::Int(key) = key {
      if *key < 0 {
        None
      } else {
        Some(
          self
            .array
            .get(*key as usize)
            .cloned()
            .unwrap_or(Value::Null),
        )
      }
    } else {
      None
    }
  }

  pub fn push(&mut self, value: &Value) {
    self.array.push(value.clone());
  }
}

impl fmt::Display for Array {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[")?;
    for (index, value) in self.array.iter().enumerate() {
      if index < self.array.len() - 1 {
        write!(f, "{}, ", value)?;
      } else {
        write!(f, "{}", value)?;
      }
    }
    write!(f, "]")
  }
}

#[derive(Debug, Finalize, Trace)]
pub struct Map {
  map: HashMap<Gc<String>, Value>,
}

impl Map {
  pub fn new() -> Map {
    Map {
      map: HashMap::new(),
    }
  }

  pub fn set(&mut self, key: Value, value: Value) -> Option<()> {
    if let Value::String(key) = &key {
      self.map.insert(Gc::clone(key), value);
      Some(())
    } else {
      None
    }
  }

  pub fn get(&self, key: &Value) -> Option<Value> {
    if let Value::String(key) = &key {
      Some(self.map.get(key).cloned().unwrap_or(Value::Null))
    } else {
      None
    }
  }
}

impl fmt::Display for Map {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{{")?;
    for (index, (key, value)) in self.map.iter().enumerate() {
      if index < self.map.len() - 1 {
        write!(f, "{}: {}, ", key, value)?;
      } else {
        write!(f, "{}: {}", key, value)?;
      }
    }
    write!(f, "}}")
  }
}

#[derive(Debug, Finalize, Trace)]
pub struct Tagged {
  tag: Gc<String>,
  value: Value,
}

impl Tagged {
  pub fn new(tag: Gc<String>, value: Value) -> Tagged {
    Tagged { tag, value }
  }

  pub fn tag(&self) -> &Gc<String> {
    &self.tag
  }

  pub fn value(&self) -> &Value {
    &self.value
  }
}

impl fmt::Display for Tagged {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}({})", self.tag, self.value)
  }
}

#[derive(Debug, Finalize, Trace)]
pub struct Lambda {
  arity: usize,
  chunk: Rc<Chunk>,
  locals: usize,
  upvalues: Vec<Gc<GcCell<Upvalue>>>,
}

impl Lambda {
  pub fn new(
    arity: usize,
    chunk: Rc<Chunk>,
    locals: usize,
    upvalues: Vec<Gc<GcCell<Upvalue>>>,
  ) -> Lambda {
    Lambda {
      arity,
      chunk,
      locals,
      upvalues,
    }
  }

  pub fn arity(&self) -> usize {
    self.arity
  }

  pub fn chunk(&self) -> &Rc<Chunk> {
    &self.chunk
  }

  pub fn locals(&self) -> usize {
    self.locals
  }

  pub fn upvalue(&self, index: usize) -> Option<&Gc<GcCell<Upvalue>>> {
    self.upvalues.get(index)
  }
}

#[derive(Finalize)]
pub struct NativeLambda(Box<dyn Fn(Value) -> Value>);

impl NativeLambda {
  pub fn new<F>(f: F) -> NativeLambda
  where
    F: Fn(Value) -> Value + 'static,
  {
    NativeLambda(Box::new(f))
  }

  pub fn call(&self, argument: Value) -> Value {
    (self.0)(argument)
  }
}

impl fmt::Debug for NativeLambda {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "<<native lambda>>")
  }
}

unsafe impl Trace for NativeLambda {
  unsafe_empty_trace!();
}

#[derive(Debug, Finalize, Trace)]
pub enum Upvalue {
  Open(usize),
  Closed(Value),
}

impl Upvalue {
  pub fn as_open(&self) -> Option<usize> {
    if let Upvalue::Open(index) = self {
      Some(*index)
    } else {
      None
    }
  }
}
