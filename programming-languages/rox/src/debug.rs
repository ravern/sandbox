use crate::{chunk::Chunk, opcode::*};

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
  // offset
  print!("{:04} ", offset);

  // line
  if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
    print!("   | ");
  } else {
    print!("{:>4} ", chunk.lines[offset]);
  }

  // instruction
  match chunk.code[offset] {
    OP_RETURN => simple_instruction("OP_RETURN", offset),
    OP_CONSTANT => constant_instruction("OP_CONSTANT", chunk, offset),
    OP_NIL => simple_instruction("OP_NIL", offset),
    OP_TRUE => simple_instruction("OP_TRUE", offset),
    OP_FALSE => simple_instruction("OP_FALSE", offset),
    OP_EQUAL => simple_instruction("OP_EQUAL", offset),
    OP_GREATER => simple_instruction("OP_GREATER", offset),
    OP_LESS => simple_instruction("OP_LESS", offset),
    OP_ADD => simple_instruction("OP_ADD", offset),
    OP_SUBTRACT => simple_instruction("OP_SUBTRACT", offset),
    OP_MULTIPLY => simple_instruction("OP_MULTIPLY", offset),
    OP_DIVIDE => simple_instruction("OP_DIVIDE", offset),
    OP_NOT => simple_instruction("OP_NOT", offset),
    OP_NEGATE => simple_instruction("OP_NEGATE", offset),
    OP_PRINT => simple_instruction("OP_PRINT", offset),
    OP_POP => simple_instruction("OP_POP", offset),
    OP_GET_GLOBAL => constant_instruction("OP_GET_GLOBAL", chunk, offset),
    OP_SET_GLOBAL => constant_instruction("OP_SET_GLOBAL", chunk, offset),
    OP_DEFINE_GLOBAL => constant_instruction("OP_DEFINE_GLOBAL", chunk, offset),
    instruction => {
      println!("{:<16} {:>4}", "OP_UNKNOWN", instruction);
      offset + 1
    }
  }
}

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
  println!("== {} ==", name);
  let mut offset = 0;
  while offset < chunk.code.len() {
    offset = disassemble_instruction(chunk, offset);
  }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
  println!("{}", name);
  offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
  let constant = chunk.code[offset + 1];
  let value = &chunk.constants[constant as usize];
  println!("{:<16} {:>4} {:?}", name, constant, value);
  offset + 2
}
