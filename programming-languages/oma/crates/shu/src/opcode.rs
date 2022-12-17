use num_derive::FromPrimitive;

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromPrimitive)]
pub enum Opcode {
  // Values
  Nul = 0,
  Tru,
  Fls,
  Flt,
  Int,
  Str,
  Arr,
  Map,
  Lmd,
  Nal,
  Pop,
  // Locals and upvalues
  Lod,
  Sav,
  Lou,
  Sau,
  Clu,
  // Code navigation
  Jmp,
  Jit,
  Jif,
  // Stack manipulation
  Dup,
  Swp,
  Rot,
  // Primitive operations
  Add,
  Sub,
  Mul,
  Div,
  Rem,
  Neg,
  Eql,
  Neq,
  Gtn,
  Gte,
  Ltn,
  Lte,
  Not,
  // Map/array operations
  Get,
  Set,
  Apn,
  // Tagged operations
  Tag,
  Utg,
  Gtg,
  // Lambda operations
  Cal,
  Ret,
}

impl Opcode {
  pub fn has_operand(&self) -> bool {
    if let Opcode::Flt
    | Opcode::Int
    | Opcode::Str
    | Opcode::Lmd
    | Opcode::Nal
    | Opcode::Lod
    | Opcode::Sav
    | Opcode::Lou
    | Opcode::Sau
    | Opcode::Jmp
    | Opcode::Jit
    | Opcode::Jif
    | Opcode::Cal = self
    {
      true
    } else {
      false
    }
  }
}
