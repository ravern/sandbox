use instruction::{Operand, OperationCode, OperationType};
use num_traits::ToPrimitive;
use result::Result;
use stack::Stack;

pub struct Core {
  program_bytes: Vec<u8>,
  instruction: u32,
  operand_stack: Stack,
}

impl Core {
  pub fn new(program_bytes: Vec<u8>) -> Core {
    Core {
      program_bytes,
      instruction: 0,
      operand_stack: Stack::new(),
    }
  }

  pub fn execute(mut self) -> Result<()> {
    info!("Starting execution of program");

    loop {
      info!("Executing instruction at {:#X}", self.instruction);

      let operation = self.next_program_bytes(1)?[0];
      info!("Fetched operation {:#X}", operation);
      let operation_code = OperationCode::from_operation(operation)?;
      info!(
        "Decoded operation code {:#X}",
        operation_code.to_u8().unwrap()
      );
      let operation_type = OperationType::from_operation(operation)?;
      info!(
        "Decoded operation type {:#X}",
        operation_type.to_u8().unwrap()
      );

      if !operation_code.allowed_operation_type(operation_type) {
        return Err(format_err!(
          "Invalid operation type {:#X}",
          operation_type.to_u8().unwrap()
        ));
      }

      match operation_code {
        OperationCode::Nope => {}
        OperationCode::Push => self.execute_push(operation_type)?,
        OperationCode::Pop => self.execute_pop(operation_type)?,
        OperationCode::Add => self.execute_add(operation_type)?,
        OperationCode::Sub => self.execute_sub(operation_type)?,
        OperationCode::Mul => self.execute_mul(operation_type)?,
        OperationCode::Div => self.execute_div(operation_type)?,
        OperationCode::Mod => self.execute_mod(operation_type)?,
        OperationCode::Jump => self.execute_jump(operation_type)?,
        OperationCode::Dup => self.execute_dup(operation_type)?,
        OperationCode::Eq => self.execute_eq(operation_type)?,
        OperationCode::Neq => self.execute_neq(operation_type)?,
      }

      if self.instruction as usize >= self.program_bytes.len() {
        info!("Ended execution of program");
        return Ok(());
      }
    }
  }

  fn execute_push(&mut self, operation_type: OperationType) -> Result<()> {
    let bytes = self.next_program_bytes(operation_type.len())?;
    let operand = Operand::new(bytes, operation_type);
    self.push_operand_stack(operand);
    Ok(())
  }

  fn execute_pop(&mut self, operation_type: OperationType) -> Result<()> {
    self.pop_operand_stack(operation_type)?;
    Ok(())
  }

  fn execute_add(&mut self, operation_type: OperationType) -> Result<()> {
    let (left_operand, right_operand) = self.pop_operand_stack_twice(operation_type)?;
    let result_operand = left_operand + right_operand;
    info!("Added values");
    self.push_operand_stack(result_operand);
    Ok(())
  }

  fn execute_sub(&mut self, operation_type: OperationType) -> Result<()> {
    let (left_operand, right_operand) = self.pop_operand_stack_twice(operation_type)?;
    let result_operand = right_operand - left_operand;
    info!("Subtracted first value from second value");
    self.push_operand_stack(result_operand);
    Ok(())
  }

  fn execute_mul(&mut self, operation_type: OperationType) -> Result<()> {
    let (left_operand, right_operand) = self.pop_operand_stack_twice(operation_type)?;
    let result_operand = left_operand * right_operand;
    info!("Multiplied values");
    self.push_operand_stack(result_operand);
    Ok(())
  }

  fn execute_div(&mut self, operation_type: OperationType) -> Result<()> {
    let (left_operand, right_operand) = self.pop_operand_stack_twice(operation_type)?;
    let result_operand = right_operand / left_operand;
    info!("Divided second value by first value");
    self.push_operand_stack(result_operand);
    Ok(())
  }

  fn execute_mod(&mut self, operation_type: OperationType) -> Result<()> {
    let (left_operand, right_operand) = self.pop_operand_stack_twice(operation_type)?;
    let result_operand = right_operand % left_operand;
    info!("Modded second value by first value");
    self.push_operand_stack(result_operand);
    Ok(())
  }

  fn execute_jump(&mut self, operation_type: OperationType) -> Result<()> {
    let stack_operand = self.pop_operand_stack(operation_type)?;
    let program_operand = self.next_operand(OperationType::U32)?;
    if !stack_operand.is_zero() {
      return Ok(());
    }
    let instruction = program_operand.to_u32();
    if instruction as usize >= self.program_bytes.len() {
      return Err(format_err!(
        "Invalid instruction specified {:#X}",
        instruction
      ));
    }
    self.instruction = instruction;
    info!("Jumped to instruction at {:#X}", instruction);
    Ok(())
  }

  fn execute_dup(&mut self, operation_type: OperationType) -> Result<()> {
    let operand = self.pop_operand_stack(operation_type)?;
    self.push_operand_stack(operand);
    self.push_operand_stack(operand);
    Ok(())
  }

  fn execute_eq(&mut self, operation_type: OperationType) -> Result<()> {
    let (left_operand, right_operand) = self.pop_operand_stack_twice(operation_type)?;
    let mut bytes = vec![0; operation_type.len()];
    if left_operand == right_operand {
      bytes[0] = 1;
    }
    self.push_operand_stack(Operand::new(bytes, operation_type));
    Ok(())
  }

  fn execute_neq(&mut self, operation_type: OperationType) -> Result<()> {
    let (left_operand, right_operand) = self.pop_operand_stack_twice(operation_type)?;
    let mut bytes = vec![0; operation_type.len()];
    if left_operand != right_operand {
      bytes[0] = 1;
    }
    self.push_operand_stack(Operand::new(bytes, operation_type));
    Ok(())
  }

  fn push_operand_stack(&mut self, operand: Operand) {
    self.operand_stack.push(operand.bytes());
    info!("Pushed value {}", operand);
  }

  fn pop_operand_stack(&mut self, operation_type: OperationType) -> Result<Operand> {
    let bytes = self.operand_stack.pop(operation_type.len())?;
    let operand = Operand::new(bytes, operation_type);
    info!("Popped value {}", operand);
    Ok(operand)
  }

  fn pop_operand_stack_twice(
    &mut self,
    operation_type: OperationType,
  ) -> Result<(Operand, Operand)> {
    let left_operand = self.pop_operand_stack(operation_type)?;
    let right_operand = self.pop_operand_stack(operation_type)?;
    Ok((left_operand, right_operand))
  }

  fn next_operand(&mut self, operation_type: OperationType) -> Result<Operand> {
    let bytes = self.next_program_bytes(operation_type.len())?;
    Ok(Operand::new(bytes, operation_type))
  }

  fn next_program_bytes(&mut self, len: usize) -> Result<Vec<u8>> {
    let instruction = self.instruction as usize;
    if instruction + len > self.program_bytes.len() {
      return Err(format_err!("Unexpected end of program"));
    }
    self.instruction += len as u32;
    Ok(self.program_bytes[instruction..instruction + len].to_vec())
  }
}
