use std::fs;
use std::str::Chars;
use anyhow::bail;

use crate::parser::Parser;
use crate::token::{Token, TokenType};
use crate::error::Error;

pub fn tokenize(filename: &String) -> anyhow::Result<()> {
  let file_contents = match fs::read_to_string(filename) {
    Ok(contents) => contents,
    Err(_) => bail!(Error::new(u8::MAX))
  };
  let mut chars = file_contents.chars();
  
  let mut line = 1;
  let mut has_error = false;
  
  let mut tokens = vec![];
  while let Some(c) = chars.next() {
    let new_tokens = parse_token(c, &mut chars, &mut line, &mut has_error);
    tokens.extend(new_tokens);
  }
  tokens.push(Token::new(TokenType::EOF, "".to_string(), None));

  for token in tokens {
    println!("{}", token);
  }

  return if has_error { bail!(Error::new(65)) } else { Ok(()) };
}

pub fn parse_token(c: char, chars: &mut Chars, line: &mut u32, has_error: &mut bool) -> Vec<Token> {
  let mut result: Vec<Token> = vec![];
  match c {
    '(' => result.push(Token::new(TokenType::LEFT_PAREN, c.to_string(), None)),
    ')' => result.push(Token::new(TokenType::RIGHT_PAREN, c.to_string(), None)),
    '{' => result.push(Token::new(TokenType::LEFT_BRACE, c.to_string(), None)),
    '}' => result.push(Token::new(TokenType::RIGHT_BRACE, c.to_string(), None)),
    ',' => result.push(Token::new(TokenType::COMMA, c.to_string(), None)),
    '.' => result.push(Token::new(TokenType::DOT, c.to_string(), None)),
    '-' => result.push(Token::new(TokenType::MINUS, c.to_string(), None)),
    '+' => result.push(Token::new(TokenType::PLUS, c.to_string(), None)),
    ';' => result.push(Token::new(TokenType::SEMICOLON, c.to_string(), None)),
    '*' => result.push(Token::new(TokenType::STAR, c.to_string(), None)),
    //'&' => result.push(Token::new(TokenType::AND, c.to_string(), None)),
    '!' => {
      let mut peekable = chars.clone().peekable();
      if peekable.next() == Some('=') {
        result.push(Token::new(TokenType::BANG_EQUAL, "!=".to_string(), None));
        chars.next();
      } else {
        result.push(Token::new(TokenType::BANG, c.to_string(), None));
      }
    }
    '=' => {
      let mut peekable = chars.clone().peekable();
      if peekable.next() == Some('=') {
        result.push(Token::new(TokenType::EQUAL_EQUAL, "==".to_string(), None));
        chars.next();
      } else {
        result.push(Token::new(TokenType::EQUAL, c.to_string(), None));
      }
    },
    '<' => {
      let mut peekable = chars.clone().peekable();
      if peekable.next() == Some('=') {
        result.push(Token::new(TokenType::LESS_EQUAL, "<=".to_string(), None));
        chars.next();
      } else {
        result.push(Token::new(TokenType::LESS, c.to_string(), None));
      }
    },
    '>' => {
      let mut peekable = chars.clone().peekable();
      if peekable.next() == Some('=') {
        result.push(Token::new(TokenType::GREATER_EQUAL, ">=".to_string(), None));
        chars.next();
      } else {
        result.push(Token::new(TokenType::GREATER, c.to_string(), None));
      }
    },
    '/' => {
      let mut peekable = chars.clone().peekable();
      if peekable.next() == Some('/') {
        while let Some(c) = chars.next() {
          if c == '\n' {
            *line += 1;
            break;
          }
        }
      } else {
        result.push(Token::new(TokenType::SLASH, c.to_string(), None));
      }
    },
    '"' => {
      let mut string = String::new();
      let mut has_end = false;
      while let Some(c) = chars.next() {
        if c == '"' {
          result.push(Token::new(TokenType::STRING, format!("\"{}\"", string), Some(string)));
          has_end = true;
          break;
        }
        string.push(c);
      }
      if !has_end {
        eprintln!("[line {}] Error: Unterminated string.", line);
        *has_error = true
      }
    },
    ' ' | '\r' | '\t' => {},
    '\n' => *line += 1,
    d if d.is_digit(10) => {
      let mut cont = String::from(d);
      let mut peekable = chars.peekable();
      let mut comma_detected = false;
      let mut new_result: Vec<Token> = vec![];
      while let Some(t) = peekable.next() {
        if t.is_digit(10) {
          cont.push(t.clone());
        } else if t == '.' && !comma_detected {
          comma_detected = true;
          cont.push(t.clone());
        } else {
          new_result = parse_token(t.clone(), chars, line, has_error);
          break;
        }
      }
      if cont.ends_with('.') {
        cont.pop();
        result.push(Token::new(TokenType::NUMBER, cont.clone(), Some(format!("{}.0", cont))));
        result.push(Token::new(TokenType::DOT, ".".to_string(), None)); 
      } else {
        result.push(Token::new(TokenType::NUMBER, cont.clone(), 
        Some(if comma_detected { format!("{}", format_number(cont)) } else { format!("{}.0", cont) })));
      }
      result.extend(new_result);
  }
  'a'..='z' | 'A'..='Z' | '_' => {
    let mut cont = String::from(c);
    let mut peekable = chars.peekable();
    let mut new_result: Vec<Token> = vec![];
    while let Some(t) = peekable.next() {
      if !t.is_alphanumeric() && t != '_' {
        new_result = parse_token(t.clone(), chars, line, has_error);
        break;
      }
      cont.push(t.clone());
    }
    let keyword = parse_keywords(cont.clone());
    if !keyword.is_empty() {
      result.extend(keyword);
    }
    else{
      result.push(Token::new(TokenType::IDENTIFIER, cont, None));
    }
    result.extend(new_result);
  }
  _ => {
    eprintln!("[line {}] Error: Unexpected character: {}", line, c);
    *has_error = true
  },
  }
  return result;
}

fn format_number(mut num_str: String) -> String {
  if !num_str.contains('.') { return num_str; }

  while let Some(c) = num_str.chars().next_back() {
    if c == '0' {
      num_str.pop();
    } else {
      break;
    }
  }
  if num_str.ends_with('.') {
    num_str.push('0');
  }
  return num_str
}

fn parse_keywords(keyword: String) -> Vec<Token> {
  let mut result = vec![];
  match keyword.as_str() {
    "print" => result.push(Token::new(TokenType::PRINT, keyword, None)),
    "and" => result.push(Token::new(TokenType::AND, keyword, None)),
    "or" => result.push(Token::new(TokenType::OR, keyword, None)),
    "for" => result.push(Token::new(TokenType::FOR, keyword, None)),
    "if" => result.push(Token::new(TokenType::IF, keyword, None)),
    "else" => result.push(Token::new(TokenType::ELSE, keyword, None)),
    "while" => result.push(Token::new(TokenType::WHILE, keyword, None)),
    "fun" => result.push(Token::new(TokenType::FUN, keyword, None)),
    "return" => result.push(Token::new(TokenType::RETURN, keyword, None)),
    "class" => result.push(Token::new(TokenType::CLASS, keyword, None)),
    "this" => result.push(Token::new(TokenType::THIS, keyword, None)),
    "super" => result.push(Token::new(TokenType::SUPER, keyword, None)),
    "true" => result.push(Token::new(TokenType::TRUE, keyword, None)),
    "false" => result.push(Token::new(TokenType::FALSE, keyword, None)),
    "nil" => result.push(Token::new(TokenType::NIL, keyword, None)),
    "var" => result.push(Token::new(TokenType::VAR, keyword, None)),
    _ => {}
  }
  return result;
}