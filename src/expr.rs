use crate::token::Token;

pub enum LiteralValue {
  Number(f64),
  String(String),
  Boolean(bool),
  Nil,
}

impl LiteralValue {
  pub fn to_string(self) -> String {
    use LiteralValue::*;
    return match self {
      Number(n) => format!("{}", n),
      String(s) => format!("\"{}\"", s),
      Boolean(b) => format!("{}", b),
      Nil => format!("nil"),
    }
  }
}

pub enum Expr {
  Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
  Grouping { expression: Box<Expr>, },
  Literal { value: LiteralValue },
  Unary { operator: Token, right: Box<Expr> },
}

impl Expr {
  pub fn to_string(self) -> String {
    use Expr::*;
    return match self {
      Binary { left, operator, right } => format!("({} {} {})", left.to_string(), operator.lexeme, right.to_string()),
      Grouping { expression } => format!("({})", expression.to_string()),
      Literal { value } => value.to_string(),
      Unary { operator, right } => format!("({}{})", operator.lexeme, right.to_string()),
    }
  }
  pub fn print(self) {
    println!("{}", self.to_string());
  }
}