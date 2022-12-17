use std::{
  cmp::{max, min},
  fs,
  path::{Path, PathBuf},
  rc::Rc,
};

use intern::Intern;

use crate::error::CompileError;

#[derive(Clone, Debug, PartialEq)]
pub struct Span {
  source: Source,
  start: usize,
  end: usize,
}

impl Span {
  pub fn combine(left: &Self, right: &Self) -> Self {
    Span::new(
      left.source.clone(),
      min(left.start, right.start),
      max(left.end, right.end),
    )
  }

  // TODO: Make this a lazy_static!.
  pub fn empty() -> Self {
    Self {
      source: Source::empty(),
      start: 0,
      end: 0,
    }
  }

  pub fn new(source: Source, start: usize, end: usize) -> Self {
    Self { source, start, end }
  }

  pub fn content(&self) -> &str {
    &self.source.content()[self.start..self.end]
  }

  pub fn line_content(&self) -> &str {
    let mut offset = self.start;
    let lines = self.source.content().split("\n");
    for line in lines {
      if offset < line.len() + 1 {
        return &self.source.content()
          [self.start - offset..self.start - offset + line.len()];
      }
      offset -= line.len() + 1;
    }
    unreachable!();
  }

  pub fn path(&self) -> &Path {
    self.source.path()
  }

  pub fn source(&self) -> &Source {
    &self.source
  }

  pub fn start(&self) -> usize {
    self.start
  }

  pub fn end(&self) -> usize {
    self.end
  }

  pub fn len(&self) -> usize {
    self.end - self.start
  }

  pub fn line(&self) -> usize {
    let mut count = 1;
    let mut offset = self.start;
    let lines = self.source.content().split("\n");
    for line in lines {
      if offset < line.len() + 1 {
        return count;
      }
      count += 1;
      offset -= line.len() + 1;
    }
    unreachable!();
  }

  pub fn column(&self) -> usize {
    let mut offset = self.start;
    let lines = self.source.content().split("\n");
    for line in lines {
      if offset < line.len() + 1 {
        return offset;
      }
      offset -= line.len() + 1;
    }
    unreachable!();
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Source {
  path: &'static PathBuf,
  content: &'static String,
}

impl Source {
  pub fn empty() -> Self {
    Self {
      path: PathBuf::from(r"/tmp/empty").intern(),
      content: "".intern(),
    }
  }

  pub fn from_file(path: &Path) -> Result<Self, CompileError> {
    let content = fs::read_to_string(path)
      .map_err(|error| CompileError::Io(Rc::new(error)))?;
    Ok(Self {
      path: path.intern(),
      content: content.intern(),
    })
  }

  pub fn from_str(content: &str, path: &Path) -> Self {
    Self {
      path: path.intern(),
      content: content.intern(),
    }
  }

  pub fn content(&self) -> &str {
    self.content.as_str()
  }

  pub fn path(&self) -> &Path {
    self.path.as_path()
  }
}
