use crate::{lex::Token, source::Span};

#[derive(Clone, Debug)]
pub struct Module {
  pub body: Expr,
}

impl Module {
  pub fn span(&self) -> Span {
    self.body.span()
  }
}

#[derive(Clone, Debug)]
pub enum Expr {
  // Values
  Lit(Lit),
  Ident(Ident),
  Map(MapExpr),
  Array(ArrayExpr),
  Lambda(LambdaExpr),
  Tag(TagExpr),
  // Execution
  Block(BlockExpr),
  Binary(BinaryExpr),
  Unary(UnaryExpr),
  Bind(BindExpr),
  Assign(AssignExpr),
  Call(CallExpr),
  Access(AccessExpr),
  // Control flow and loops
  If(IfExpr),
  Case(CaseExpr),
  For(ForExpr),
  While(WhileExpr),
}

impl Expr {
  pub fn span(&self) -> Span {
    match self {
      Self::Lit(lit) => lit.span(),
      Self::Ident(ident) => ident.span(),
      Self::Map(map_expr) => map_expr.span(),
      Self::Array(array_expr) => array_expr.span(),
      Self::Lambda(lambda_expr) => lambda_expr.span(),
      Self::Tag(tag_expr) => tag_expr.span(),
      Self::Block(block_expr) => block_expr.span(),
      Self::Binary(binary_expr) => binary_expr.span(),
      Self::Unary(unary_expr) => unary_expr.span(),
      Self::Bind(bind_expr) => bind_expr.span(),
      Self::Assign(assign_expr) => assign_expr.span(),
      Self::Call(call_expr) => call_expr.span(),
      Self::Access(access_expr) => access_expr.span(),
      Self::If(if_expr) => if_expr.span(),
      Self::Case(case_expr) => case_expr.span(),
      Self::For(for_expr) => for_expr.span(),
      Self::While(while_expr) => while_expr.span(),
    }
  }
}

#[derive(Clone, Debug)]
pub struct MapExpr {
  pub span: Span,
  pub pairs: Vec<MapExprPair>,
}

impl MapExpr {
  pub fn span(&self) -> Span {
    self.span.clone()
  }
}

#[derive(Clone, Debug)]
pub enum MapExprPair {
  Spread(Expr),
  Expr(Expr, Expr),
  Ident(Ident, Expr),
}

#[derive(Clone, Debug)]
pub struct ArrayExpr {
  pub span: Span,
  pub items: Vec<ArrayExprItem>,
}

impl ArrayExpr {
  pub fn span(&self) -> Span {
    self.span.clone()
  }
}

#[derive(Clone, Debug)]
pub enum ArrayExprItem {
  Spread(Expr),
  Expr(Expr),
}

#[derive(Clone, Debug)]
pub struct TagExpr {
  pub span: Span, // span needs to include close paren
  pub tag: Ident,
  pub expr: Box<Expr>,
}

impl TagExpr {
  pub fn span(&self) -> Span {
    self.span.clone()
  }
}

#[derive(Clone, Debug)]
pub struct BlockExpr {
  pub span: Span,
  pub exprs: Vec<Expr>,
  pub has_semi: bool,
}

impl BlockExpr {
  pub fn span(&self) -> Span {
    self.span.clone()
  }
}

#[derive(Clone, Debug)]
pub struct BinaryExpr {
  pub op: BinaryOp,
  pub left: Box<Expr>,
  pub right: Box<Expr>,
}

impl BinaryExpr {
  pub fn span(&self) -> Span {
    Span::combine(&self.left.span(), &self.right.span())
  }
}

#[derive(Clone, Debug)]
pub enum BinaryOp {
  Add,
  Subtract,
  Multiply,
  Divide,
  And,
  Or,
  Equal,
  NotEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,
}

impl BinaryOp {
  pub fn from_token(token: Token) -> Option<Self> {
    let op = match token {
      Token::And => Self::And,
      Token::Or => Self::Or,
      Token::EqualEqual => Self::Equal,
      Token::BangEqual => Self::NotEqual,
      Token::Greater => Self::Greater,
      Token::GreaterEqual => Self::GreaterEqual,
      Token::Less => Self::Less,
      Token::LessEqual => Self::LessEqual,
      Token::Plus => Self::Add,
      Token::Dash => Self::Subtract,
      Token::Star => Self::Multiply,
      Token::Slash => Self::Divide,
      _ => return None,
    };
    Some(op)
  }
}

#[derive(Clone, Debug)]
pub struct UnaryExpr {
  pub span: Span,
  pub op: UnaryOp,
  pub operand: Box<Expr>,
}

impl UnaryExpr {
  pub fn span(&self) -> Span {
    self.span.clone()
  }
}

#[derive(Clone, Debug)]
pub enum UnaryOp {
  Negate,
  Not,
}

impl UnaryOp {
  pub fn from_token(token: Token) -> Option<Self> {
    let op = match token {
      Token::Dash => Self::Negate,
      Token::Bang => Self::Not,
      _ => return None,
    };
    Some(op)
  }
}

#[derive(Clone, Debug)]
pub struct BindExpr {
  pub bindee: Pat,
  pub value: Box<Expr>,
}

impl BindExpr {
  pub fn span(&self) -> Span {
    Span::combine(&self.bindee.span(), &self.value.span())
  }
}

#[derive(Clone, Debug)]
pub struct AssignExpr {
  pub assignee: AssignExprAssignee,
  pub value: Box<Expr>,
}

impl AssignExpr {
  pub fn span(&self) -> Span {
    Span::combine(&self.assignee.span(), &self.value.span())
  }
}

#[derive(Clone, Debug)]
pub enum AssignExprAssignee {
  Pat(Pat),
  Access(AccessExpr),
}

impl AssignExprAssignee {
  pub fn from_expr(expr: Expr) -> Option<Self> {
    match expr {
      Expr::Access(access_expr) => Some(Self::Access(access_expr)),
      expr => Pat::from_expr(expr).map(Self::Pat),
    }
  }

  pub fn span(&self) -> Span {
    match self {
      AssignExprAssignee::Pat(pat) => pat.span(),
      AssignExprAssignee::Access(access_expr) => access_expr.span(),
    }
  }
}

#[derive(Clone, Debug)]
pub struct IfExpr {
  pub span: Span,
  pub condition: Box<Expr>,
  pub body: Box<Expr>,
  pub otherwise: Option<Box<Expr>>,
}

impl IfExpr {
  pub fn span(&self) -> Span {
    self.span.clone()
  }
}

#[derive(Clone, Debug)]
pub struct CaseExpr {
  pub span: Span,
  pub subject: Box<Expr>,
  pub arms: Vec<(Pat, Expr)>,
}

impl CaseExpr {
  pub fn span(&self) -> Span {
    self.span.clone()
  }
}

#[derive(Clone, Debug)]
pub struct ForExpr {
  pub span: Span,
  pub item: Pat,
  pub iterator: Box<Expr>,
  pub body: Box<Expr>,
}

impl ForExpr {
  pub fn span(&self) -> Span {
    self.span.clone()
  }
}

#[derive(Clone, Debug)]
pub struct WhileExpr {
  pub span: Span,
  pub condition: Box<Expr>,
  pub body: Box<Expr>,
}

impl WhileExpr {
  pub fn span(&self) -> Span {
    self.span.clone()
  }
}

#[derive(Clone, Debug)]
pub struct LambdaExpr {
  pub span: Span,
  pub parameters: Vec<LambdaExprParameter>,
  pub body: Box<Expr>,
}

impl LambdaExpr {
  pub fn span(&self) -> Span {
    self.span.clone()
  }
}

#[derive(Clone, Debug)]
pub enum LambdaExprParameter {
  Spread(Ident),
  Pat(Pat),
}

#[derive(Clone, Debug)]
pub struct CallExpr {
  pub receiver: Box<Expr>,
  pub arguments: Vec<CallExprArgument>,
}

impl CallExpr {
  pub fn span(&self) -> Span {
    let mut span = self.receiver.span();
    for argument in self.arguments.iter() {
      span = Span::combine(&span, &argument.span());
    }
    span
  }
}

#[derive(Clone, Debug)]
pub enum CallExprArgument {
  Spread(Expr),
  Expr(Expr),
}

impl CallExprArgument {
  pub fn span(&self) -> Span {
    match self {
      Self::Spread(expr) => expr.span(),
      Self::Expr(expr) => expr.span(),
    }
  }
}

#[derive(Clone, Debug)]
pub struct AccessExpr {
  pub receiver: Box<Expr>,
  pub field: AccessExprField,
}

impl AccessExpr {
  // TODO: This is just a placeholder to get code to build.
  pub fn span(&self) -> Span {
    self.receiver.span()
  }
}

#[derive(Clone, Debug)]
pub enum AccessExprField {
  Ident(Ident),
  Expr(Box<Expr>),
}

#[derive(Clone, Debug)]
pub enum Pat {
  Lit(Lit),
  Ident(Ident),
  Tag(TagPat),
  Map(MapPat),
  Array(ArrayPat),
}

impl Pat {
  pub fn from_expr(expr: Expr) -> Option<Self> {
    let pat = match expr {
      Expr::Lit(lit) => Self::Lit(lit),
      Expr::Ident(ident) => Self::Ident(ident),
      Expr::Tag(TagExpr { span, tag, expr }) => {
        if let Some(pat) = Self::from_expr(*expr) {
          Pat::Tag(TagPat {
            span,
            tag,
            pat: Box::new(pat),
          })
        } else {
          return None;
        }
      }
      Expr::Map(MapExpr {
        span,
        pairs: expr_pairs,
      }) => {
        let mut pat_pairs = Vec::new();
        for expr_pair in expr_pairs {
          match expr_pair {
            MapExprPair::Spread(expr) => {
              if let Expr::Ident(ident) = expr {
                pat_pairs.push(MapPatPair::Spread(ident));
              } else {
                return None;
              }
            }
            MapExprPair::Ident(field, expr_value) => {
              if let Some(pat_value) = Self::from_expr(expr_value) {
                pat_pairs.push(MapPatPair::Ident(field, pat_value));
              } else {
                return None;
              }
            }
            MapExprPair::Expr(..) => return None,
          }
        }
        Self::Map(MapPat {
          span,
          pairs: pat_pairs,
        })
      }
      Expr::Array(ArrayExpr {
        span,
        items: expr_items,
      }) => {
        let mut pat_items = Vec::new();
        for expr_item in expr_items {
          match expr_item {
            ArrayExprItem::Spread(expr) => {
              if let Expr::Ident(ident) = expr {
                pat_items.push(ArrayPatItem::Spread(ident));
              } else {
                return None;
              }
            }
            ArrayExprItem::Expr(expr) => {
              if let Some(pat) = Self::from_expr(expr) {
                pat_items.push(ArrayPatItem::Pat(pat));
              } else {
                return None;
              }
            }
          }
        }
        Self::Array(ArrayPat {
          span,
          items: pat_items,
        })
      }
      _ => return None,
    };
    Some(pat)
  }

  pub fn idents(&self) -> Vec<Ident> {
    let mut idents = Vec::new();

    match self {
      Self::Lit(_) => {}
      Self::Ident(ident) => {
        idents.push(ident.clone());
      }
      Self::Tag(tag_pat) => {
        idents.extend(tag_pat.pat.idents());
      }
      Self::Map(map_pat) => {
        for pair in &map_pat.pairs {
          match pair {
            MapPatPair::Ident(_, pat) => {
              idents.extend(pat.idents());
            }
            _ => unimplemented!(),
          }
        }
      }
      _ => unimplemented!(),
    }

    idents
  }

  pub fn span(&self) -> Span {
    match self {
      Self::Lit(lit) => lit.span(),
      Self::Ident(ident) => ident.span(),
      Self::Tag(tag_pat) => tag_pat.span(),
      Self::Map(map_pat) => map_pat.span(),
      Self::Array(array_pat) => array_pat.span(),
    }
  }
}

#[derive(Clone, Debug)]
pub struct TagPat {
  pub span: Span,
  pub tag: Ident,
  pub pat: Box<Pat>,
}

impl TagPat {
  pub fn span(&self) -> Span {
    self.span.clone()
  }
}

#[derive(Clone, Debug)]
pub struct MapPat {
  pub span: Span,
  pub pairs: Vec<MapPatPair>,
}

impl MapPat {
  pub fn span(&self) -> Span {
    self.span.clone()
  }
}

#[derive(Clone, Debug)]
pub enum MapPatPair {
  Spread(Ident),
  Ident(Ident, Pat),
}

#[derive(Clone, Debug)]
pub struct ArrayPat {
  pub span: Span,
  pub items: Vec<ArrayPatItem>,
}

impl ArrayPat {
  pub fn span(&self) -> Span {
    self.span.clone()
  }
}

#[derive(Clone, Debug)]
pub enum ArrayPatItem {
  Spread(Ident),
  Pat(Pat),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ident {
  pub span: Span,
  pub content: &'static String,
}

impl Ident {
  pub fn span(&self) -> Span {
    self.span.clone()
  }
}

#[derive(Clone, Debug)]
pub enum Lit {
  Number(NumberLit),
  Bool(BoolLit),
  String(StringLit),
  Null(Span),
}

impl Lit {
  pub fn span(&self) -> Span {
    match self {
      Self::Number(number_lit) => number_lit.span.clone(),
      Self::Bool(bool_lit) => bool_lit.span.clone(),
      Self::String(string_lit) => string_lit.span.clone(),
      Self::Null(span) => span.clone(),
    }
  }
}

#[derive(Clone, Debug)]
pub struct IntLit {
  pub span: Span,
  pub int: i64,
}

#[derive(Clone, Debug)]
pub struct NumberLit {
  pub span: Span,
  pub number: f64,
}

#[derive(Clone, Debug)]
pub struct BoolLit {
  pub span: Span,
  pub bool: bool,
}

#[derive(Clone, Debug)]
pub struct StringLit {
  pub span: Span,
  pub string: &'static String,
}

impl StringLit {
  pub fn span(&self) -> Span {
    self.span.clone()
  }
}

struct Context {
  locals: usize,
  total_locals: usize,
  scopes: Vec<usize>,
}

impl Context {
  fn new() -> Context {
    Context {
      locals: 0,
      total_locals: 0,
      scopes: Vec::new(),
    }
  }

  fn add(&mut self, count: usize) {
    self.locals += count;
    if self.locals > self.total_locals {
      self.total_locals = self.locals;
    }
  }

  fn enter_scope(&mut self) {
    self.scopes.push(self.locals);
  }

  fn exit_scope(&mut self) {
    self.locals = self.scopes.pop().unwrap();
  }
}

pub fn count_expr_locals(expr: &Expr) -> usize {
  let mut ctx = Context::new();
  count_expr_locals_with_ctx(&mut ctx, expr);
  ctx.total_locals
}

fn count_expr_locals_with_ctx(ctx: &mut Context, expr: &Expr) {
  match expr {
    Expr::Block(block_expr) => {
      for expr in &block_expr.exprs {
        count_expr_locals_with_ctx(ctx, expr);
      }
    }
    Expr::Bind(bind_expr) => count_pat_locals(ctx, &bind_expr.bindee),
    Expr::Call(call_expr) => {
      for argument in &call_expr.arguments {
        match argument {
          CallExprArgument::Expr(expr) => count_expr_locals_with_ctx(ctx, expr),
          _ => unimplemented!(),
        }
      }
    }
    Expr::If(if_expr) => {
      ctx.enter_scope();
      count_expr_locals_with_ctx(ctx, &if_expr.body);
      ctx.exit_scope();
      if let Some(otherwise) = if_expr.otherwise.as_ref() {
        ctx.enter_scope();
        count_expr_locals_with_ctx(ctx, otherwise);
        ctx.exit_scope();
      }
    }
    Expr::Case(case_expr) => {
      for (pat, expr) in &case_expr.arms {
        ctx.enter_scope();
        count_pat_locals(ctx, pat);
        count_expr_locals_with_ctx(ctx, expr);
        ctx.exit_scope();
      }
    }
    Expr::While(while_expr) => {
      ctx.enter_scope();
      count_expr_locals_with_ctx(ctx, &while_expr.body);
      ctx.exit_scope();
    }
    Expr::Lit(_) => {}
    Expr::Ident(_) => {}
    Expr::Binary(_) => {}
    Expr::Tag(_) => {}
    Expr::Array(_) => {}
    Expr::Access(_) => {}
    Expr::Assign(_) => {}
    Expr::Lambda(_) => {}
    expr => {
      dbg!(expr);
      unimplemented!()
    }
  }
}

fn count_pat_locals(ctx: &mut Context, pat: &Pat) {
  match pat {
    Pat::Ident(_) => ctx.add(1),
    Pat::Tag(tag_pat) => count_pat_locals(ctx, &tag_pat.pat),
    Pat::Map(map_pat) => {
      for pair in &map_pat.pairs {
        match pair {
          MapPatPair::Ident(_, pat) => count_pat_locals(ctx, &pat),
          _ => unimplemented!(),
        }
      }
    }
    Pat::Lit(_) => {}
    _ => unimplemented!(),
  }
}
