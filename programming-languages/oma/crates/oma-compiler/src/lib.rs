use std::path::Path;

use self::{
  emit::emit, error::CompileError, gen::Generator, parse::Parser,
  source::Source,
};

mod ast;
mod chunk;
mod debug;
mod emit;
mod error;
mod gen;
mod lex;
mod parse;
mod source;

pub fn compile(
  registry: Vec<(&'static str, usize)>,
) -> Result<Vec<u8>, CompileError> {
  let parser = Parser::new(Source::from_str(
    "let counter = 0;
    counter = counter + 1;
    while counter < 100000 {
      __console_info(counter);
      counter = counter + 1;
    }
",
    Path::new("main.oma"),
  ));
  let module = parser.parse()?;
  let generator = Generator::new(registry);
  let chunk = generator.generate(module)?;
  Ok(emit(chunk))
}
