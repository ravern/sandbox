use std::{cell::RefCell, ops::Add, rc::Rc};

#[derive(Debug, PartialEq)]
pub enum Value {
  Number(f64),
  Bool(bool),
  Obj(Rc<RefCell<Obj>>),
  Nil,
}

impl Value {
  pub fn from_string(string: &str) -> Value {
    Value::Obj(Rc::new(RefCell::new(Obj::String(internship::IStr::new(
      string,
    )))))
  }

  pub fn is_falsey(&self) -> bool {
    match self {
      Value::Nil => true,
      Value::Bool(false) => true,
      _ => false,
    }
  }

  pub fn is_string(&self) -> bool {
    if let Value::Obj(obj) = self {
      obj.borrow().is_string()
    } else {
      false
    }
  }

  pub fn to_string(&self) -> internship::IStr {
    if let Value::Obj(obj) = self {
      obj.borrow().to_string()
    } else {
      panic!("value is not a string");
    }
  }
}

impl Clone for Value {
  fn clone(&self) -> Self {
    match self {
      Value::Number(number) => Value::Number(*number),
      Value::Bool(bool) => Value::Bool(*bool),
      Value::Obj(obj) => Value::Obj(Rc::clone(obj)),
      Value::Nil => Value::Nil,
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum Obj {
  String(internship::IStr),
}

impl Obj {
  fn is_string(&self) -> bool {
    if let Obj::String(_) = self {
      true
    } else {
      false
    }
  }

  fn to_string(&self) -> internship::IStr {
    match self {
      Obj::String(string) => string.clone(),
      _ => panic!("object is not a string"),
    }
  }
}
