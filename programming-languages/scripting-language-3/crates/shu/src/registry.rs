use gc::Gc;

use crate::value::{NativeLambda, Value};

pub struct NativeLambdaRegistry {
  native_lambdas: Vec<Gc<NativeLambda>>,
}

impl NativeLambdaRegistry {
  pub fn new() -> NativeLambdaRegistry {
    NativeLambdaRegistry {
      native_lambdas: Vec::new(),
    }
  }

  pub fn add<F>(&mut self, lambda: F) -> usize
  where
    F: Fn(Value) -> Value + 'static,
  {
    self.native_lambdas.push(Gc::new(NativeLambda::new(lambda)));
    self.native_lambdas.len() - 1
  }

  pub fn get(&self, id: usize) -> Option<&Gc<NativeLambda>> {
    self.native_lambdas.get(id)
  }
}
