use result::Result;

pub struct Stack {
  stack: Vec<u8>,
}

impl Stack {
  pub fn new() -> Stack {
    Stack { stack: Vec::new() }
  }

  pub fn push(&mut self, mut bytes: Vec<u8>) {
    self.stack.append(&mut bytes);
  }

  pub fn pop(&mut self, len: usize) -> Result<Vec<u8>> {
    let stack_len = self.stack.len();
    if len > stack_len {
      return Err(format_err!("Failed to pop empty operand stack"));
    }
    let bytes = self.stack[stack_len - len..stack_len].to_vec();
    self.stack.truncate(stack_len - len);
    Ok(bytes)
  }
}
