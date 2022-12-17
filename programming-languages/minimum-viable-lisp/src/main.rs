use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::{IntoIterator, Iterator};
use std::rc::Rc;

fn is_whitespace(char: u8) -> bool {
  char == b' ' || char == b'\n' || char == b'\t' || char == b'\r'
}

fn is_digit(char: u8) -> bool {
  char >= b'0' && char <= b'9'
}

fn is_alphabetic(char: u8) -> bool {
  char >= b'a' && char <= b'z'
}

#[derive(Clone, Debug)]
enum Value {
  List(List),
  Number(f64),
  Symbol(String),
  Lambda(Rc<Lambda>),
  NativeLambda(fn(Env, List) -> Result<Value, EvalError>),
}

#[derive(Clone, Debug)]
enum List {
  Cons(Rc<Cons>),
  Nil,
}

impl List {
  fn push(&self, value: Value) -> List {
    match self {
      List::Cons(cons) => List::Cons(Rc::new(Cons {
        car: cons.car.clone(),
        cdr: cons.cdr.push(value),
      })),
      List::Nil => List::Cons(Rc::new(Cons {
        car: value,
        cdr: List::Nil,
      })),
    }
  }
}

impl IntoIterator for List {
  type Item = Value;
  type IntoIter = IntoIter;

  fn into_iter(self) -> Self::IntoIter {
    IntoIter(self)
  }
}

struct IntoIter(List);

impl Iterator for IntoIter {
  type Item = Value;

  fn next(&mut self) -> Option<Self::Item> {
    match &self.0 {
      List::Cons(cons) => {
        let value = cons.car.clone();
        self.0 = cons.cdr.clone();
        Some(value)
      }
      List::Nil => None,
    }
  }
}

#[derive(Debug)]
struct Cons {
  car: Value,
  cdr: List,
}

#[derive(Debug)]
struct Lambda {
  env: Env,
  params: Vec<String>,
  body: Value,
}

#[derive(Debug)]
enum ReadError {
  Char(u8),
  Eof,
}

fn bite_list(source: &str) -> Result<(Value, usize), ReadError> {
  let chars = source.as_bytes();
  let mut current = 1;
  let mut list = List::Nil;

  loop {
    match chars.get(current) {
      Some(b')') => {
        current += 1;
        return Ok((Value::List(list), current));
      }
      Some(char) if is_whitespace(*char) => current += 1,
      Some(_) => {
        let (value, len) = bite(&source[current..])?;
        current += len;
        list = list.push(value);
      }
      None => return Err(ReadError::Eof),
    }
  }
}

fn bite_number(source: &str) -> Result<(Value, usize), ReadError> {
  let chars = source.as_bytes();
  let mut current = 0;
  let mut has_decimal = false;
  let mut buf = Vec::new();

  loop {
    match chars.get(current) {
      Some(char) if is_digit(*char) => {
        current += 1;
        buf.push(*char);
      }
      Some(b'.') if has_decimal => return Err(ReadError::Char(b'.')),
      Some(b'.') => {
        current += 1;
        has_decimal = true;
        buf.push(b'.');
      }
      Some(b')') => break,
      Some(char) if is_whitespace(*char) => break,
      None => break,
      Some(char) => return Err(ReadError::Char(*char)),
    }
  }

  Ok((
    Value::Number(String::from_utf8(buf).unwrap().parse().unwrap()),
    current,
  ))
}

fn bite_symbol(source: &str) -> Result<(Value, usize), ReadError> {
  let chars = source.as_bytes();
  let mut current = 0;
  let mut buf = Vec::new();

  loop {
    match chars.get(current) {
      Some(char) if is_alphabetic(*char) => {
        current += 1;
        buf.push(*char);
      }
      Some(b')') => break,
      Some(char) if is_whitespace(*char) => break,
      None => break,
      Some(char) => return Err(ReadError::Char(*char)),
    }
  }

  Ok((Value::Symbol(String::from_utf8(buf).unwrap()), current))
}

fn bite(source: &str) -> Result<(Value, usize), ReadError> {
  match source.as_bytes().first() {
    Some(char) if is_whitespace(*char) => bite(&source[1..]),
    Some(b'+') => Ok((Value::Symbol("+".into()), 1)),
    Some(b'-') => Ok((Value::Symbol("-".into()), 1)),
    Some(b'*') => Ok((Value::Symbol("*".into()), 1)),
    Some(b'/') => Ok((Value::Symbol("/".into()), 1)),
    Some(b'(') => bite_list(source),
    Some(char) if is_digit(*char) => bite_number(source),
    Some(char) if is_alphabetic(*char) => bite_symbol(source),
    Some(char) => return Err(ReadError::Char(*char)),
    None => return Err(ReadError::Eof),
  }
}

fn read(source: &str) -> Result<Value, ReadError> {
  bite(source).map(|(value, _)| value)
}

fn add(env: Env, args: List) -> Result<Value, EvalError> {
  let numbers = args
    .into_iter()
    .map(|arg| match eval(&env, arg)? {
      Value::Number(number) => Ok(number),
      _ => Err(EvalError::Type("number".into())),
    })
    .collect::<Result<Vec<f64>, EvalError>>()?;

  if numbers.len() < 2 {
    return Err(EvalError::Arity(2));
  }

  Ok(Value::Number(
    numbers.into_iter().fold(0.0, |sum, number| sum + number),
  ))
}

fn sub(env: Env, args: List) -> Result<Value, EvalError> {
  let numbers = args
    .into_iter()
    .map(|arg| match eval(&env, arg)? {
      Value::Number(number) => Ok(number),
      _ => Err(EvalError::Type("number".into())),
    })
    .collect::<Result<Vec<f64>, EvalError>>()?;

  if numbers.len() < 2 {
    return Err(EvalError::Arity(2));
  }

  let mut numbers = numbers.into_iter();
  let init = numbers.next().unwrap();

  Ok(Value::Number(
    numbers.fold(init, |sum, number| sum - number),
  ))
}

fn mul(env: Env, args: List) -> Result<Value, EvalError> {
  let numbers = args
    .into_iter()
    .map(|arg| match eval(&env, arg)? {
      Value::Number(number) => Ok(number),
      _ => Err(EvalError::Type("number".into())),
    })
    .collect::<Result<Vec<f64>, EvalError>>()?;

  if numbers.len() < 2 {
    return Err(EvalError::Arity(2));
  }

  Ok(Value::Number(
    numbers
      .into_iter()
      .fold(1.0, |result, number| result * number),
  ))
}

fn def(env: Env, args: List) -> Result<Value, EvalError> {
  let args = args
    .into_iter()
    .map(|arg| eval(&env, arg))
    .collect::<Result<Vec<Value>, EvalError>>()?;

  if args.len() != 2 {
    return Err(EvalError::Arity(2));
  }

  let symbol = match args.get(0).unwrap() {
    Value::Symbol(symbol) => symbol.clone(),
    _ => return Err(EvalError::Type("symbol".into())),
  };

  env
    .globals
    .borrow_mut()
    .insert(symbol, args.get(1).unwrap().clone());

  Ok(Value::List(List::Nil))
}

fn let_(env: Env, args: List) -> Result<Value, EvalError> {
  let args = args.into_iter().collect::<Vec<Value>>();

  if args.len() != 2 {
    return Err(EvalError::Arity(2));
  }

  let definitions = match args.get(0).unwrap().clone() {
    Value::List(list) => list.into_iter().collect::<Vec<Value>>(),
    _ => return Err(EvalError::Type("list".into())),
  };

  let mut locals = HashMap::new();
  for definition in definitions {
    let definition = match definition {
      Value::List(list) => list.into_iter().collect::<Vec<Value>>(),
      _ => return Err(EvalError::Type("list".into())),
    };
    let symbol = match definition.get(0).unwrap() {
      Value::Symbol(symbol) => symbol.clone(),
      _ => return Err(EvalError::Type("list".into())),
    };
    let value = definition.get(1).unwrap().clone();
    locals.insert(symbol, value);
  }

  eval(
    &Env::with_parent(&env, locals),
    args.get(1).unwrap().clone(),
  )
}

fn eq(env: Env, args: List) -> Result<Value, EvalError> {
  let args = args
    .into_iter()
    .map(|arg| eval(&env, arg))
    .collect::<Result<Vec<Value>, EvalError>>()?;

  if args.len() != 2 {
    return Err(EvalError::Arity(2));
  }

  let left = args.get(0).unwrap();
  let right = args.get(1).unwrap();

  match (left, right) {
    (Value::Number(left), Value::Number(right)) if left == right => {
      Ok(Value::Symbol("true".into()))
    }
    (Value::Symbol(left), Value::Symbol(right)) if left == right => {
      Ok(Value::Symbol("true".into()))
    }
    (Value::List(List::Cons(left)), Value::List(List::Cons(right)))
      if Rc::ptr_eq(left, right) =>
    {
      Ok(Value::Symbol("true".into()))
    }
    (Value::List(List::Nil), Value::List(List::Nil)) => {
      Ok(Value::Symbol("true".into()))
    }
    _ => Ok(Value::List(List::Nil)),
  }
}

fn quote(_env: Env, args: List) -> Result<Value, EvalError> {
  let args = args.into_iter().collect::<Vec<Value>>();

  if args.len() != 1 {
    return Err(EvalError::Arity(1));
  }

  Ok(args.get(0).unwrap().clone())
}

fn car(env: Env, args: List) -> Result<Value, EvalError> {
  let args = args
    .into_iter()
    .map(|arg| eval(&env, arg))
    .collect::<Result<Vec<Value>, EvalError>>()?;

  if args.len() != 1 {
    return Err(EvalError::Arity(2));
  }

  let cons = match args.get(0).unwrap() {
    Value::List(List::Cons(cons)) => cons,
    _ => return Err(EvalError::Type("list".into())),
  };

  Ok(cons.car.clone())
}

fn cdr(env: Env, args: List) -> Result<Value, EvalError> {
  let args = args
    .into_iter()
    .map(|arg| eval(&env, arg))
    .collect::<Result<Vec<Value>, EvalError>>()?;

  if args.len() != 1 {
    return Err(EvalError::Arity(2));
  }

  let cons = match args.get(0).unwrap() {
    Value::List(List::Cons(cons)) => cons,
    _ => return Err(EvalError::Type("list".into())),
  };

  Ok(Value::List(cons.cdr.clone()))
}

fn cons(env: Env, args: List) -> Result<Value, EvalError> {
  let args = args
    .into_iter()
    .map(|arg| eval(&env, arg))
    .collect::<Result<Vec<Value>, EvalError>>()?;

  if args.len() != 2 {
    return Err(EvalError::Arity(2));
  }

  let car = args.get(0).unwrap().clone();

  let cdr = match args.get(1).unwrap() {
    Value::List(list) => list,
    _ => return Err(EvalError::Type("list".into())),
  }
  .clone();

  Ok(Value::List(List::Cons(Rc::new(Cons { car, cdr }))))
}

fn lambda(env: Env, args: List) -> Result<Value, EvalError> {
  let args = args.into_iter().collect::<Vec<Value>>();

  if args.len() != 2 {
    return Err(EvalError::Arity(2));
  }

  let params = match &args[0] {
    Value::List(list) => list
      .clone()
      .into_iter()
      .map(|arg| match arg {
        Value::Symbol(symbol) => Ok(symbol),
        _ => Err(EvalError::Type("symbol".into())),
      })
      .collect::<Result<Vec<String>, EvalError>>()?,
    _ => return Err(EvalError::Type("list".into())),
  };

  Ok(Value::Lambda(Rc::new(Lambda {
    env: Env::with_parent(&env, HashMap::new()),
    params,
    body: args[1].clone(),
  })))
}

fn cond(env: Env, args: List) -> Result<Value, EvalError> {
  let args = args.into_iter().collect::<Vec<Value>>();

  if args.is_empty() {
    return Err(EvalError::Arity(1));
  }

  for arg in args {
    let pair = match arg {
      Value::List(list) => list.into_iter().collect::<Vec<Value>>(),
      _ => return Err(EvalError::Type("non-empty list".into())),
    };

    if pair.len() != 2 {
      return Err(EvalError::Arity(2));
    }

    let test = pair.get(0).unwrap().clone();
    let action = pair.get(1).unwrap().clone();

    if let Value::List(List::Nil) = eval(&env, test)? {
      continue;
    }

    return eval(&env, action);
  }

  Ok(Value::List(List::Nil))
}

#[derive(Clone, Debug)]
struct Env {
  globals: Rc<RefCell<HashMap<String, Value>>>,
  locals: HashMap<String, Value>,
}

impl Env {
  fn empty() -> Env {
    let mut globals = HashMap::new();

    globals.insert("+".to_string(), Value::NativeLambda(add));
    globals.insert("-".to_string(), Value::NativeLambda(sub));
    globals.insert("*".to_string(), Value::NativeLambda(mul));
    globals.insert("def".to_string(), Value::NativeLambda(def));
    globals.insert("let".to_string(), Value::NativeLambda(let_));
    globals.insert("quote".to_string(), Value::NativeLambda(quote));
    globals.insert("eq".to_string(), Value::NativeLambda(eq));
    globals.insert("car".to_string(), Value::NativeLambda(car));
    globals.insert("cdr".to_string(), Value::NativeLambda(cdr));
    globals.insert("cons".to_string(), Value::NativeLambda(cons));
    globals.insert("lambda".to_string(), Value::NativeLambda(lambda));
    globals.insert("cond".to_string(), Value::NativeLambda(cond));

    Env {
      globals: Rc::new(RefCell::new(globals)),
      locals: HashMap::new(),
    }
  }

  fn with_parent(parent: &Env, mut locals: HashMap<String, Value>) -> Env {
    locals.extend(parent.locals.clone());
    Env {
      globals: Rc::clone(&parent.globals),
      locals,
    }
  }
}

#[derive(Debug)]
enum EvalError {
  Undef(String),
  Type(String),
  Arity(usize),
  Nil,
}

fn eval(env: &Env, value: Value) -> Result<Value, EvalError> {
  match value {
    Value::Number(number) => Ok(Value::Number(number)),
    Value::Symbol(symbol) => {
      if let Some(local) = env.locals.get(&symbol) {
        Ok(local.clone())
      } else if let Some(global) = env.globals.borrow().get(&symbol) {
        Ok(global.clone())
      } else {
        Err(EvalError::Undef(symbol))
      }
    }
    Value::List(list) => eval_list(env, list),
    _ => unreachable!(),
  }
}

fn eval_list(env: &Env, list: List) -> Result<Value, EvalError> {
  let cons = match list {
    List::Nil => return Err(EvalError::Nil),
    List::Cons(cons) => cons,
  };

  match eval(env, cons.car.clone())? {
    Value::Lambda(lambda) => {
      let args = cons.cdr.clone().into_iter().collect::<Vec<Value>>();

      if lambda.params.len() != args.len() {
        return Err(EvalError::Arity(lambda.params.len()));
      }

      let mut locals = HashMap::new();
      for index in 0..lambda.params.len() {
        let param = lambda.params.get(index).unwrap().clone();
        let arg = eval(&env, args.get(index).unwrap().clone())?;
        locals.insert(param, arg);
      }

      eval(&Env::with_parent(&lambda.env, locals), lambda.body.clone())
    }
    Value::NativeLambda(lambda) => lambda(env.clone(), cons.cdr.clone()),
    _ => Err(EvalError::Type("lambda".into())),
  }
}

#[derive(Debug)]
enum Error {
  Read(ReadError),
  Eval(EvalError),
}

fn interpret(source: &str) -> Result<Value, Error> {
  let value = read(source).map_err(Error::Read)?;
  eval(&Env::empty(), value).map_err(Error::Eval)
}

fn main() {
  println!(
    "{:?}",
    interpret("
((lambda (y)
  ((y (lambda (fac) (lambda (n) (cond ((eq n 1) 1) ((quote true) (* n (fac (- n 1)))))))) 5))
  (lambda (f) ((lambda (x) (x x)) (lambda (x) (f (lambda (y) ((x x) y)))))))
    ")
  );
}
