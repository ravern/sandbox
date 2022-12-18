use crate::pos::Pos;

#[derive(Debug, Fail)]
pub enum Error {
  #[fail(display = "ParseError: {}\n  at {}:{}", message, path, pos)]
  Parse {
    message: String,
    path: String,
    pos: Pos,
  },
}

impl Error {
  pub fn new_parse<S, T>(message: S, path: T, pos: Pos) -> Error
  where
    S: Into<String>,
    T: Into<String>,
  {
    Error::Parse {
      message: message.into(),
      path: path.into(),
      pos,
    }
  }
}
