use std::collections::HashMap;

use array_init::array_init;
use thiserror::Error;

use crate::{
  chunk::Chunk, compile::Compiler, debug::disassemble_instruction, opcode::*,
  value::Value,
};

mod chunk;
mod compile;
mod debug;
mod opcode;
mod value;

pub const STACK_MAX: usize = 1024;

#[derive(Debug, Error)]
pub enum Error {
  #[error("failed to compile")]
  Compile,
  #[error("failed to run")]
  Runtime,
}

pub struct Vm {
  chunk: Option<Chunk>,
  globals: HashMap<internship::IStr, Value>,
  ip: usize,
  stack: [Value; STACK_MAX],
  stack_top: usize,
}

impl Vm {
  pub fn new() -> Vm {
    Vm {
      chunk: None,
      globals: HashMap::new(),
      ip: 0,
      stack: array_init(|_| Value::Number(0.0)),
      stack_top: 0,
    }
  }

  pub fn interpret(&mut self, source: &str) -> Result<(), Error> {
    let mut chunk = Chunk::new();

    let mut compiler = Compiler::new(source);
    if !compiler.compile(&mut chunk) {
      return Err(Error::Compile);
    }

    self.ip = 0;
    self.chunk = Some(chunk);

    self.run()
  }

  fn run(&mut self) -> Result<(), Error> {
    loop {
      // print!("          ");
      // for index in 0..self.stack_top {
      //   print!("[ {:?} ]", self.stack[index]);
      // }
      // println!("");
      // disassemble_instruction(self.chunk(), self.ip);
      match self.step() {
        OP_RETURN => {
          return Ok(());
        }
        OP_CONSTANT => {
          let index = self.step();
          let constant = self.chunk().constants[index as usize].clone();
          self.push(constant);
        }
        OP_NIL => self.push(Value::Nil),
        OP_TRUE => self.push(Value::Bool(true)),
        OP_FALSE => self.push(Value::Bool(false)),
        OP_EQUAL => {
          let right = self.pop();
          let left = self.pop();
          self.push(Value::Bool(right == left))
        }
        OP_GREATER => match (self.pop(), self.pop()) {
          (Value::Number(right), Value::Number(left)) => {
            self.push(Value::Bool(left > right));
          }
          _ => {
            self.runtime_error("operands must be a numbers");
            return Err(Error::Runtime);
          }
        },
        OP_LESS => match (self.pop(), self.pop()) {
          (Value::Number(right), Value::Number(left)) => {
            self.push(Value::Bool(left < right));
          }
          _ => {
            self.runtime_error("operands must be a numbers");
            return Err(Error::Runtime);
          }
        },
        OP_ADD => match (self.pop(), self.pop()) {
          (Value::Number(right), Value::Number(left)) => {
            self.push(Value::Number(left + right));
          }
          (right, left) if right.is_string() && left.is_string() => {
            self.push(Value::from_string(
              format!("{}{}", left.to_string(), right.to_string()).as_str(),
            ));
          }
          _ => {
            self.runtime_error("operands must be a numbers");
            return Err(Error::Runtime);
          }
        },
        OP_SUBTRACT => match (self.pop(), self.pop()) {
          (Value::Number(right), Value::Number(left)) => {
            self.push(Value::Number(left - right));
          }
          _ => {
            self.runtime_error("operands must be a numbers");
            return Err(Error::Runtime);
          }
        },
        OP_MULTIPLY => match (self.pop(), self.pop()) {
          (Value::Number(right), Value::Number(left)) => {
            self.push(Value::Number(left * right));
          }
          _ => {
            self.runtime_error("operands must be a numbers");
            return Err(Error::Runtime);
          }
        },
        OP_DIVIDE => match (self.pop(), self.pop()) {
          (Value::Number(right), Value::Number(left)) => {
            self.push(Value::Number(left / right));
          }
          _ => {
            self.runtime_error("operands must be a numbers");
            return Err(Error::Runtime);
          }
        },
        OP_NOT => {
          let value = self.pop();
          self.push(Value::Bool(value.is_falsey()))
        }
        OP_NEGATE => match self.pop() {
          Value::Number(number) => self.push(Value::Number(-number)),
          _ => {
            self.runtime_error("operand must be a number");
            return Err(Error::Runtime);
          }
        },
        OP_PRINT => {
          println!("{:?}", self.pop());
        }
        OP_POP => {
          self.pop();
        }
        OP_SET_GLOBAL => {
          let index = self.step();
          let name = self.chunk().constants[index as usize].to_string();
          let value = self.peek(0);
          if let None = self.globals.insert(name.clone(), value) {
            self.globals.remove(&name);
            self.runtime_error(&format!("undefined variable '{}'", name));
            return Err(Error::Runtime);
          }
        }
        OP_GET_GLOBAL => {
          let index = self.step();
          let name = self.chunk().constants[index as usize].to_string();
          let value = match self.globals.get(&name) {
            Some(value) => value.clone(),
            None => {
              self.runtime_error(&format!("undefined variable '{}'", name));
              return Err(Error::Runtime);
            }
          };
          self.push(value);
        }
        OP_DEFINE_GLOBAL => {
          let index = self.step();
          let name = self.chunk().constants[index as usize].to_string();
          let value = self.pop();
          self.globals.insert(name, value);
        }
        _ => {}
      }
    }
  }

  fn chunk(&self) -> &Chunk {
    self.chunk.as_ref().expect("chunk not set")
  }

  fn step(&mut self) -> u8 {
    let code = self.chunk().code[self.ip];
    self.ip += 1;
    code
  }

  fn push(&mut self, value: Value) {
    self.stack[self.stack_top] = value;
    self.stack_top += 1;
  }

  fn pop(&mut self) -> Value {
    self.stack_top -= 1;
    self.stack[self.stack_top].clone()
  }

  fn peek(&mut self, distance: usize) -> Value {
    self.stack[self.stack_top - 1 - distance].clone()
  }

  fn runtime_error(&mut self, msg: &str) {
    eprintln!("{}", msg);

    let line = self.chunk().lines[self.ip - 1];
    eprintln!("[line {}] in script", line);
    self.reset_stack();
  }

  fn reset_stack(&mut self) {
    self.stack_top = 0;
  }
}
