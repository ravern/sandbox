use std::rc::Rc;

use crate::chunk::Chunk;

#[derive(Debug)]
pub struct Frame {
  pub chunk: Rc<Chunk>,
  pub ip: usize,
  pub bp: usize,
}
