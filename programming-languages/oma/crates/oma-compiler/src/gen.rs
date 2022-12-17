use std::mem;

use intern::Intern;

use crate::{
  ast::*,
  chunk::{Chunk, Function, Op, Opcode, Operand},
  error::{CompileError, VerifyError},
  source::Span,
};

pub struct Generator {
  registry: Vec<(&'static str, usize)>,
  context: Context,
}

impl Generator {
  pub fn new(registry: Vec<(&'static str, usize)>) -> Generator {
    Generator {
      registry,
      context: Context::new(),
    }
  }

  pub fn generate(self, module: Module) -> Result<Function, CompileError> {
    self.module(module)
  }

  fn module(mut self, module: Module) -> Result<Function, CompileError> {
    let locals = count_expr_locals(&module.body) as u64;

    self.expr(module.body)?;
    self.emit_ret();

    Ok(Function {
      arity: 0,
      chunk: self.context.chunk,
      locals,
      upvalues: Vec::new(),
    })
  }

  fn expr(&mut self, expr: Expr) -> Result<(), CompileError> {
    match expr {
      Expr::Lit(lit) => self.lit(lit),
      Expr::Ident(ident) => self.ident(ident),
      Expr::Map(map_expr) => self.map_expr(map_expr),
      Expr::Array(array_expr) => self.array_expr(array_expr),
      Expr::Lambda(lambda_expr) => self.lambda_expr(lambda_expr),
      Expr::Tag(tag_expr) => self.tag_expr(tag_expr),
      Expr::Block(block_expr) => self.block_expr(block_expr),
      Expr::Binary(binary_expr) => self.binary_expr(binary_expr),
      Expr::Unary(unary_expr) => self.unary_expr(unary_expr),
      Expr::Bind(bind_expr) => self.bind_expr(bind_expr),
      Expr::Assign(assign_expr) => self.assign_expr(assign_expr),
      Expr::Call(call_expr) => self.call_expr(call_expr),
      Expr::Access(access_expr) => self.access_expr(access_expr),
      Expr::If(if_expr) => self.if_expr(if_expr),
      Expr::Case(case_expr) => self.case_expr(case_expr),
      Expr::While(while_expr) => self.while_expr(while_expr),
      _ => unimplemented!(),
    }
  }

  fn map_expr(&mut self, map_expr: MapExpr) -> Result<(), CompileError> {
    self.context.emit(Op::new(Opcode::Map));

    for pair in map_expr.pairs {
      match pair {
        MapExprPair::Ident(key, value) => {
          self.context.emit(Op::new(Opcode::Dup));
          self.expr(value)?;
          self.context.emit(Op::with_operand(
            Opcode::Str,
            Operand::String(key.content.to_string()),
          ));
          self.context.emit(Op::new(Opcode::Set));
          self.context.emit(Op::new(Opcode::Pop));
        }
        _ => unimplemented!(),
      }
    }

    Ok(())
  }

  fn array_expr(&mut self, array_expr: ArrayExpr) -> Result<(), CompileError> {
    self.context.emit(Op::new(Opcode::Arr));

    for item in array_expr.items {
      match item {
        ArrayExprItem::Expr(expr) => {
          self.context.emit(Op::new(Opcode::Dup));
          self.expr(expr)?;
          self.context.emit(Op::new(Opcode::Apn));
          self.context.emit(Op::new(Opcode::Pop));
        }
        _ => unimplemented!(),
      }
    }

    Ok(())
  }

  fn lambda_expr(
    &mut self,
    lambda_expr: LambdaExpr,
  ) -> Result<(), CompileError> {
    let arity = lambda_expr.parameters.len() as u64;

    let locals = count_expr_locals(&lambda_expr.body);

    let context = mem::replace(&mut self.context, Context::new());
    self.context = Context::with_parent(context);

    for parameter in lambda_expr.parameters.iter() {
      match parameter {
        LambdaExprParameter::Pat(Pat::Ident(ident)) => {
          self.context.add_local(&ident);
        }
        _ => unimplemented!(),
      }
    }

    self.expr(*lambda_expr.body)?;
    self.emit_ret();

    let context = mem::replace(&mut self.context, Context::new());
    self.context = *context.parent.unwrap();

    let chunk = context.chunk;

    let upvalues = context
      .upvalues
      .into_iter()
      .map(|upvalue| (upvalue.index as u64, upvalue.is_local))
      .collect();

    let function = Function {
      arity,
      chunk,
      locals: locals as u64,
      upvalues,
    };

    self.emit_lmd(function);

    Ok(())
  }

  fn tag_expr(&mut self, tag_expr: TagExpr) -> Result<(), CompileError> {
    self.expr(*tag_expr.expr)?;

    self.emit_tag(tag_expr.tag.content);

    self.context.emit(Op::new(Opcode::Tag));

    Ok(())
  }

  fn block_expr(&mut self, block_expr: BlockExpr) -> Result<(), CompileError> {
    let mut exprs = block_expr.exprs;

    let last_expr = if let Some(expr) = exprs.pop() {
      expr
    } else {
      self.emit_nul();
      return Ok(());
    };

    for expr in exprs {
      self.expr(expr)?;
      self.context.emit(Op::new(Opcode::Clu));
    }

    self.expr(last_expr)?;
    if block_expr.has_semi {
      self.context.emit(Op::new(Opcode::Clu));
      self.emit_nul();
    }

    Ok(())
  }

  fn binary_expr(
    &mut self,
    binary_expr: BinaryExpr,
  ) -> Result<(), CompileError> {
    self.expr(*binary_expr.left)?;
    self.expr(*binary_expr.right)?;

    let opcode = match binary_expr.op {
      BinaryOp::Add => Opcode::Add,
      BinaryOp::Subtract => Opcode::Sub,
      BinaryOp::Multiply => Opcode::Mul,
      BinaryOp::Divide => Opcode::Div,
      BinaryOp::And => unimplemented!(),
      BinaryOp::Or => unimplemented!(),
      BinaryOp::Equal => Opcode::Eql,
      BinaryOp::NotEqual => Opcode::Neq,
      BinaryOp::Greater => Opcode::Gtn,
      BinaryOp::GreaterEqual => Opcode::Gte,
      BinaryOp::Less => Opcode::Ltn,
      BinaryOp::LessEqual => Opcode::Lte,
    };
    self.context.emit(Op::new(opcode));

    Ok(())
  }

  fn unary_expr(&mut self, unary_expr: UnaryExpr) -> Result<(), CompileError> {
    self.expr(*unary_expr.operand)?;

    let opcode = match unary_expr.op {
      UnaryOp::Negate => Opcode::Neg,
      UnaryOp::Not => Opcode::Not,
    };
    self.context.emit(Op::new(opcode));

    Ok(())
  }

  fn bind_expr(&mut self, bind_expr: BindExpr) -> Result<(), CompileError> {
    self.expr(*bind_expr.value)?;

    self.bind_expr_pat(bind_expr.bindee)?;

    Ok(())
  }

  fn bind_expr_pat(&mut self, pat: Pat) -> Result<(), CompileError> {
    match pat {
      Pat::Ident(ident) => {
        let local = self.context.add_local(&ident);
        self
          .context
          .emit(Op::with_operand(Opcode::Sav, Operand::Usize(local)));
      }

      Pat::Tag(tag_pat) => {
        self.context.emit(Op::new(Opcode::Dup));
        self.context.emit(Op::new(Opcode::Utg));
        self.bind_expr_pat(*tag_pat.pat)?;
        self.context.emit(Op::new(Opcode::Pop));
      }

      Pat::Map(map_pat) => {
        for pair in map_pat.pairs {
          match pair {
            MapPatPair::Ident(ident, pat) => {
              self.context.emit(Op::new(Opcode::Dup));
              self.context.emit(Op::with_operand(
                Opcode::Str,
                Operand::String(ident.content.to_string()),
              ));
              self.context.emit(Op::new(Opcode::Get));
              self.bind_expr_pat(pat)?;
              self.context.emit(Op::new(Opcode::Pop));
            }
            _ => unimplemented!(),
          }
        }
      }
      _ => unimplemented!(),
    }

    Ok(())
  }

  fn assign_expr(
    &mut self,
    assign_expr: AssignExpr,
  ) -> Result<(), CompileError> {
    match assign_expr.assignee {
      AssignExprAssignee::Pat(pat) => {
        self.expr(*assign_expr.value)?;
        self.assign_expr_pat(pat)?;
      }
      AssignExprAssignee::Access(access_expr) => {
        self.expr(*access_expr.receiver)?;

        self.expr(*assign_expr.value)?;

        match access_expr.field {
          AccessExprField::Expr(expr) => self.expr(*expr)?,
          AccessExprField::Ident(ident) => {
            self.context.emit(Op::with_operand(
              Opcode::Str,
              Operand::String(ident.content.to_string()),
            ));
          }
        }

        self.context.emit(Op::new(Opcode::Set));
      }
    }

    Ok(())
  }

  fn assign_expr_pat(&mut self, pat: Pat) -> Result<(), CompileError> {
    match pat {
      Pat::Ident(ident) => {
        let local = self.context.local(&ident).ok_or(CompileError::Verify(
          VerifyError::unresolved_identifier(ident.span()),
        ))?;
        self
          .context
          .emit(Op::with_operand(Opcode::Sav, Operand::Usize(local)));
      }

      Pat::Tag(tag_pat) => {
        self.context.emit(Op::new(Opcode::Dup));
        self.context.emit(Op::new(Opcode::Utg));
        self.assign_expr_pat(*tag_pat.pat)?;
        self.context.emit(Op::new(Opcode::Pop));
      }

      Pat::Map(map_pat) => {
        for pair in map_pat.pairs {
          match pair {
            MapPatPair::Ident(ident, pat) => {
              self.context.emit(Op::new(Opcode::Dup));
              self.context.emit(Op::with_operand(
                Opcode::Str,
                Operand::String(ident.content.to_string()),
              ));
              self.context.emit(Op::new(Opcode::Get));
              self.assign_expr_pat(pat)?;
              self.context.emit(Op::new(Opcode::Pop));
            }
            _ => unimplemented!(),
          }
        }
      }
      _ => unimplemented!(),
    }

    Ok(())
  }

  fn call_expr(&mut self, call_expr: CallExpr) -> Result<(), CompileError> {
    self.expr(*call_expr.receiver)?;

    let arity = call_expr.arguments.len();
    for argument in call_expr.arguments {
      match argument {
        CallExprArgument::Expr(expr) => self.expr(expr)?,
        CallExprArgument::Spread(_) => unimplemented!(),
      }
    }

    self
      .context
      .emit(Op::with_operand(Opcode::Cal, Operand::Usize(arity)));

    Ok(())
  }

  fn access_expr(
    &mut self,
    access_expr: AccessExpr,
  ) -> Result<(), CompileError> {
    self.expr(*access_expr.receiver)?;

    match access_expr.field {
      AccessExprField::Expr(expr) => self.expr(*expr)?,
      AccessExprField::Ident(ident) => {
        self.context.emit(Op::with_operand(
          Opcode::Str,
          Operand::String(ident.content.to_string()),
        ));
      }
    }

    self.context.emit(Op::new(Opcode::Get));

    Ok(())
  }

  fn if_expr(&mut self, if_expr: IfExpr) -> Result<(), CompileError> {
    self.expr(*if_expr.condition)?;

    let jump_if_offset = self
      .context
      .emit(Op::with_operand(Opcode::Jif, Operand::Usize(usize::MAX)));

    self.expr(*if_expr.body)?;

    let jump_offset = self.context.emit(Op::new(Opcode::Jmp));

    self
      .context
      .patch(jump_if_offset, Operand::Usize(self.context.len()));

    if let Some(otherwise) = if_expr.otherwise {
      self.expr(*otherwise)?;
    } else {
      self.emit_nul();
    }

    self
      .context
      .patch(jump_offset, Operand::Usize(self.context.len()));

    Ok(())
  }

  fn case_expr(&mut self, case_expr: CaseExpr) -> Result<(), CompileError> {
    self.expr(*case_expr.subject)?;

    let mut jump_offsets = Vec::new();
    let mut jump_if_offsets = Vec::new();

    for (pat, expr) in case_expr.arms {
      for jump_if_offset in &jump_if_offsets {
        self
          .context
          .patch(*jump_if_offset, Operand::Usize(self.context.len()));
      }
      jump_if_offsets.clear();

      self.context.enter_scope();

      self.case_expr_pat(&mut jump_if_offsets, Vec::new(), pat)?;

      self.expr(expr)?;

      jump_offsets.push(
        self
          .context
          .emit(Op::with_operand(Opcode::Jmp, Operand::Usize(usize::MAX))),
      );

      self.context.exit_scope();
    }

    for jump_if_offset in &jump_if_offsets {
      self
        .context
        .patch(*jump_if_offset, Operand::Usize(self.context.len()));
    }
    jump_if_offsets.clear();

    self.emit_nul();

    for offset in jump_offsets {
      self
        .context
        .patch(offset, Operand::Usize(self.context.len()));
    }

    self.context.emit(Op::new(Opcode::Swp));
    self.context.emit(Op::new(Opcode::Pop));

    Ok(())
  }

  fn case_expr_pat(
    &mut self,
    jump_if_offsets: &mut Vec<usize>,
    path: Vec<String>,
    pat: Pat,
  ) -> Result<(), CompileError> {
    match pat {
      Pat::Ident(ident) => {
        let local = self.context.add_local(&ident);
        self.context.emit(Op::new(Opcode::Dup));
        self.case_expr_pat_subject(path.clone());
        self
          .context
          .emit(Op::with_operand(Opcode::Sav, Operand::Usize(local)));
        self.context.emit(Op::new(Opcode::Pop));
      }

      Pat::Tag(tag_pat) => {
        self.context.emit(Op::new(Opcode::Dup));
        self.case_expr_pat_subject(path.clone());
        self.context.emit(Op::new(Opcode::Gtg));
        self.context.emit(Op::with_operand(
          Opcode::Str,
          Operand::String(tag_pat.tag.content.to_string()),
        ));
        self.context.emit(Op::new(Opcode::Eql));
        jump_if_offsets.push(
          self
            .context
            .emit(Op::with_operand(Opcode::Jif, Operand::Usize(usize::MAX))),
        );

        let mut path = path.clone();
        path.push("__UTG".to_string());
        self.case_expr_pat(jump_if_offsets, path, *tag_pat.pat)?;
      }

      Pat::Map(map_pat) => {
        for pair in map_pat.pairs {
          match pair {
            MapPatPair::Ident(ident, pat) => {
              self.context.emit(Op::new(Opcode::Dup));
              self.case_expr_pat_subject(path.clone());
              self.context.emit(Op::with_operand(
                Opcode::Str,
                Operand::String(ident.content.to_string()),
              ));
              self.context.emit(Op::new(Opcode::Get));
              self.emit_nul();
              self.context.emit(Op::new(Opcode::Eql));
              jump_if_offsets.push(self.context.emit(Op::with_operand(
                Opcode::Jit,
                Operand::Usize(usize::MAX),
              )));

              let mut path = path.clone();
              path.push(ident.content.to_string());
              self.case_expr_pat(jump_if_offsets, path, pat)?;
            }
            _ => unimplemented!(),
          }
        }
      }

      Pat::Lit(lit) => {
        self.context.emit(Op::new(Opcode::Dup));
        self.case_expr_pat_subject(path.clone());
        self.lit(lit)?;
        self.context.emit(Op::new(Opcode::Eql));
        jump_if_offsets.push(
          self
            .context
            .emit(Op::with_operand(Opcode::Jif, Operand::Usize(usize::MAX))),
        );
      }

      _ => unimplemented!(),
    }

    Ok(())
  }

  fn case_expr_pat_subject(&mut self, path: Vec<String>) {
    for component in path {
      if component == "__UTG" {
        self.context.emit(Op::new(Opcode::Utg));
      } else {
        self
          .context
          .emit(Op::with_operand(Opcode::Str, Operand::String(component)));
        self.context.emit(Op::new(Opcode::Get));
      }
    }
  }

  fn while_expr(&mut self, while_expr: WhileExpr) -> Result<(), CompileError> {
    let offset = self.context.len();

    self.expr(*while_expr.condition)?;

    let jump_if_offset = self
      .context
      .emit(Op::with_operand(Opcode::Jif, Operand::Usize(usize::MAX)));

    self.expr(*while_expr.body)?;

    self
      .context
      .emit(Op::with_operand(Opcode::Jmp, Operand::Usize(offset)));

    self
      .context
      .patch(jump_if_offset, Operand::Usize(self.context.len()));

    self.emit_nul();

    Ok(())
  }

  fn lit(&mut self, lit: Lit) -> Result<(), CompileError> {
    match lit {
      Lit::Number(NumberLit { number: float, .. }) => self
        .context
        .emit(Op::with_operand(Opcode::Flt, Operand::F64(float))),
      Lit::Bool(BoolLit { bool: true, .. }) => {
        self.context.emit(Op::new(Opcode::Tru))
      }
      Lit::Bool(BoolLit { bool: false, .. }) => {
        self.context.emit(Op::new(Opcode::Fls))
      }
      Lit::String(StringLit { string, .. }) => self.context.emit(
        Op::with_operand(Opcode::Str, Operand::String(string.to_string())),
      ),
      Lit::Null(_) => self.emit_nul(),
    };
    Ok(())
  }

  fn ident(&mut self, ident: Ident) -> Result<(), CompileError> {
    if let Some(local) = self.context.local(&ident) {
      self.emit_lod(local);
    } else if let Some(upvalue) = self.context.upvalue(&ident) {
      self.emit_lou(upvalue);
    } else if let Some((_, id)) = self
      .registry
      .iter()
      .find(|(name, _)| name == ident.content)
      .copied()
    {
      self.emit_nal(id);
    } else {
      return Err(CompileError::Verify(VerifyError::unresolved_identifier(
        ident.span,
      )));
    }

    Ok(())
  }

  fn emit_tag(&mut self, name: &'static String) -> usize {
    self.context.emit(Op::with_operand(
      Opcode::Str,
      Operand::String(name.to_string()),
    ))
  }

  fn emit_lmd(&mut self, function: Function) -> usize {
    self
      .context
      .emit(Op::with_operand(Opcode::Lmd, Operand::Function(function)))
  }

  fn emit_nal(&mut self, id: usize) -> usize {
    self
      .context
      .emit(Op::with_operand(Opcode::Nal, Operand::Usize(id)))
  }

  fn emit_lod(&mut self, local: usize) -> usize {
    self
      .context
      .emit(Op::with_operand(Opcode::Lod, Operand::Usize(local)))
  }

  fn emit_lou(&mut self, upvalue: usize) -> usize {
    self
      .context
      .emit(Op::with_operand(Opcode::Lou, Operand::Usize(upvalue)))
  }

  fn emit_ret(&mut self) -> usize {
    self.context.emit(Op::new(Opcode::Ret))
  }

  fn emit_nul(&mut self) -> usize {
    self.context.emit(Op::new(Opcode::Nul))
  }
}

struct Context {
  parent: Option<Box<Context>>,
  locals: Vec<(Ident, usize)>,
  local_depth: usize,
  upvalues: Vec<Upvalue>,
  chunk: Chunk,
}

impl Context {
  fn new() -> Self {
    Self {
      parent: None,
      locals: vec![(
        Ident {
          content: "__lambda".intern(),
          span: Span::empty(),
        },
        0,
      )],
      local_depth: 0,
      upvalues: Vec::new(),
      chunk: Chunk::new(),
    }
  }

  fn with_parent(parent: Self) -> Self {
    Self {
      parent: Some(Box::new(parent)),
      locals: vec![(
        Ident {
          content: "__lambda".intern(),
          span: Span::empty(),
        },
        0,
      )],
      local_depth: 0,
      upvalues: Vec::new(),
      chunk: Chunk::new(),
    }
  }

  fn emit(&mut self, op: Op) -> usize {
    dbg!(&op);
    self.chunk.emit(op)
  }

  fn patch(&mut self, offset: usize, operand: Operand) {
    self.chunk.patch(offset, operand)
  }

  fn add_local(&mut self, ident: &Ident) -> usize {
    self.locals.push((ident.clone(), self.local_depth));
    self.locals.len() - 1
  }

  fn local(&self, ident: &Ident) -> Option<usize> {
    for index in (0..self.locals.len()).rev() {
      let (local_ident, _) = &self.locals[index];
      if local_ident.content == ident.content {
        return Some(index);
      }
    }
    None
  }

  fn add_upvalue(&mut self, upvalue: Upvalue) -> usize {
    if let Some(index) = self.upvalues.iter().position(|u| u == &upvalue) {
      index
    } else {
      self.upvalues.push(upvalue);
      self.upvalues.len() - 1
    }
  }

  fn upvalue(&mut self, ident: &Ident) -> Option<usize> {
    if let Some(parent) = &mut self.parent {
      if let Some(local) = parent.local(ident) {
        return Some(self.add_upvalue(Upvalue {
          index: local,
          is_local: true,
        }));
      } else if let Some(upvalue) = parent.upvalue(ident) {
        return Some(self.add_upvalue(Upvalue {
          index: upvalue,
          is_local: false,
        }));
      }
    }
    None
  }

  fn enter_scope(&mut self) {
    self.local_depth += 1;
  }

  fn exit_scope(&mut self) -> usize {
    let mut pop_count = 0;

    self.locals = self
      .locals
      .iter()
      .filter(|(_, depth)| {
        if depth >= &self.local_depth {
          pop_count += 1;
          false
        } else {
          true
        }
      })
      .map(|(local, depth)| (local.clone(), *depth))
      .collect();

    self.local_depth -= 1;

    pop_count
  }

  fn len(&self) -> usize {
    self.chunk.len()
  }
}

#[derive(PartialEq)]
struct Upvalue {
  index: usize,
  is_local: bool,
}
