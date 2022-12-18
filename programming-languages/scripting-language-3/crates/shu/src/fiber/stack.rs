use gc::{Gc, GcCell};

use crate::{
  error::Trace,
  value::{Upvalue, Value},
};

use super::frame::Frame;

#[derive(Debug)]
pub struct Stack {
  frames: Vec<Frame>,
  pub values: Vec<Value>,
  // TODO: Change this to use a linked list for faster insertion?
  upvalues: Vec<Gc<GcCell<Upvalue>>>,
}

impl Stack {
  pub fn new() -> Stack {
    Stack {
      frames: Vec::new(),
      values: Vec::new(),
      upvalues: Vec::new(),
    }
  }

  pub fn push(&mut self, value: Value) {
    self.values.push(value);
  }

  pub fn pop(&mut self) -> Option<Value> {
    self.values.pop()
  }

  pub fn last(&self) -> Option<&Value> {
    self.values.last()
  }

  pub fn get(&self, local: usize) -> Option<&Value> {
    self.values.get(local)
  }

  pub fn set(&mut self, local: usize, value: Value) -> Option<()> {
    if let Some(local_value) = self.values.get_mut(local) {
      *local_value = value;
      Some(())
    } else {
      None
    }
  }

  pub fn len(&self) -> usize {
    self.values.len()
  }

  pub fn push_frame(&mut self, frame: Frame) {
    self.frames.push(frame)
  }

  pub fn pop_frame(&mut self) -> Option<Frame> {
    self.frames.pop()
  }

  pub fn is_frames_empty(&self) -> bool {
    self.frames.is_empty()
  }

  // Searches for an existing upvalue pointing to the same slot on the stack. Creates
  // the upvalue if it cannot be found.
  // TODO: Improve searching by keeping `self.upvalues` sorted.
  pub fn upvalue(&mut self, index: usize) -> Gc<GcCell<Upvalue>> {
    if let Some(upvalue) = self.upvalues.iter().find(|upvalue| {
      upvalue
        .borrow()
        .as_open()
        .map(|upvalue_index| upvalue_index == index)
        .unwrap_or(false)
    }) {
      Gc::clone(upvalue)
    } else {
      let upvalue = Gc::new(GcCell::new(Upvalue::Open(index)));
      self.upvalues.push(Gc::clone(&upvalue));
      upvalue
    }
  }

  // Pops and searches for an existing upvalue that pointed to that value. If found,
  // closes the upvalue (setting it to be the popped value).
  // TODO: Improve searching by keeping `self.upvalues` sorted.
  pub fn close_upvalue(&mut self) -> Option<()> {
    let value = if let Some(value) = self.values.pop() {
      value
    } else {
      return None;
    };

    let index = self.values.len();

    if let Some(position) = self.upvalues.iter().position(|upvalue| {
      upvalue
        .borrow()
        .as_open()
        .map(|upvalue_index| upvalue_index == index)
        .unwrap_or(false)
    }) {
      let upvalue = self.upvalues.remove(position);
      *upvalue.borrow_mut() = Upvalue::Closed(value);
    }

    Some(())
  }

  // Adds the current stack's context onto the trace
  pub fn build_trace(&self) -> Trace {
    let mut trace = Trace::new();
    for frame in self.frames.iter() {
      trace.push(frame.chunk.info().and_then(|info| info.span(frame.ip - 1)))
    }
    trace
  }
}
