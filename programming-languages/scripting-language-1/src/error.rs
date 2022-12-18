use span::Span;

#[derive(Debug, Fail)]
pub enum Error {
  #[fail(display = "Failed to parse")]
  Parse { span: Span },
}
