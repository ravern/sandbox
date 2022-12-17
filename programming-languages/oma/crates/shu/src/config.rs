use crate::registry::NativeLambdaRegistry;

pub struct Config<C>
where
  C: Compiler,
{
  registry: NativeLambdaRegistry,
  compiler: C,
}

pub trait Compiler {
  type Error;

  fn compile(&self, path: &str) -> Result<Vec<u8>, Self::Error>;
}
