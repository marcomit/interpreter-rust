use std::fmt::Display;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum TokenType {
  // Single-character tokens
  LEFT_PAREN,
  RIGHT_PAREN,
  LEFT_BRACE,
  RIGHT_BRACE,
  // One or two character tokens
  COMMA,
  DOT,
  MINUS,
  PLUS,
  SEMICOLON,
  STAR,
  SLASH,
  EQUAL,
  EQUAL_EQUAL,
  BANG,
  BANG_EQUAL,
  LESS,
  LESS_EQUAL,
  GREATER,
  GREATER_EQUAL,

  // Literals
  IDENTIFIER,
  STRING,
  NUMBER,

  // Keywords
  AND,
  CLASS,
  ELSE,
  FALSE,
  FUN,
  FOR,
  IF,
  NIL,
  OR,
  PRINT,
  RETURN,
  SUPER,
  THIS,
  TRUE,
  VAR,
  WHILE,

  EOF,
}

pub struct Token {
  pub _type: TokenType,
  pub lexeme: String,
  pub value: Option<String>,
}

impl Token {
  pub fn new(_type: TokenType, lexeme: String, value: Option<String>) -> Self {
    Token {
      _type,
      lexeme,
      value,
    }
  }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {}", self._type, self.lexeme, self.value.clone().unwrap_or("null".to_string()))
    }
}