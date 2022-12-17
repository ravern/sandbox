use std::rc::Rc;

use crate::{
  error::{Error, Reason, Trace},
  fiber::{Fiber, Status},
  parse::Parser,
  registry::NativeLambdaRegistry,
};

pub struct Vm {
  registry: Option<Rc<NativeLambdaRegistry>>,
  fiber: Option<Fiber>,
}

impl Vm {
  pub fn new() -> Vm {
    Vm {
      registry: None,
      fiber: None,
    }
  }

  pub fn run(&mut self, registry: NativeLambdaRegistry, executable: &[u8]) -> Result<(), Error> {
    let parser = Parser::new(executable);
    let function = parser
      .parse()
      .map_err(|error| Error::new(Reason::Parse(error), Trace::new()))?;

    self.registry = Some(Rc::new(registry));

    self.fiber = Some(Fiber::new(
      Rc::clone(&self.registry.as_ref().unwrap()),
      function,
    ));

    while let Status::Running = self.fiber.as_mut().unwrap().step()? {}

    Ok(())
  }

  pub fn step(&mut self) -> Result<(), Error> {
    self.fiber.as_mut().unwrap().step()?;
    Ok(())
  }
}
