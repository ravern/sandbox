use span::Span;

macro_rules! node {
  (
    $struct_name:ident
    $enum_name:ident $enum_body:tt
  ) => {
    #[derive(Debug)]
    pub struct $struct_name {
      pub span: Span,
      pub kind: $enum_name,
    }

    #[derive(Debug)]
    pub enum $enum_name $enum_body
  };
  (
    $struct_name:ident($type_name:ident)
  ) => {
    #[derive(Debug)]
    pub struct $struct_name {
      span: Span,
      value: $type_name,
    }
  };
}

node! {
  Program
  ProgramKind {
    Expression(Expression),
  }
}

impl Program {
  pub fn from_expression(expression: Expression) -> Program {
    Program {
      span: expression.span,
      kind: ProgramKind::Expression(expression),
    }
  }
}

node! {
  Expression
  ExpressionKind {
    Assignment(Identifier, Box<Expression>),
    Operation(Operation),
  }
}

impl Expression {
  pub fn from_assignment(identifier: Identifier, expression: Expression) -> Expression {
    Expression {
      span: identifier.span,
      kind: ExpressionKind::Assignment(identifier, Box::new(expression)),
    }
  }

  pub fn from_operation(operation: Operation) -> Expression {
    Expression {
      span: operation.span,
      kind: ExpressionKind::Operation(operation),
    }
  }
}

node! {
  Operation
  OperationKind {
    Binary(Operator, Term, Term),
    Unary(Operator, Term),
    Term(Term),
  }
}

node! {
  Term
  TermKind {
    Identifier(Identifier),
    Number(Number),
    Paren(Box<Expression>),
  }
}

node! {
  Operator
  OperatorKind {
    And,
    Or,
    Add,
    Subtract,
    Multiply,
    Divide,
  }
}

node! {
  Identifier(String)
}

node! {
  Number(f32)
}
