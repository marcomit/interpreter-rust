use anyhow::bail;
use std::fs;
use std::str::Chars;

use crate::error::Error;
use crate::token::*;
use crate::tokenizer::parse_token;

pub struct Parser {
    tokens: Vec<Token>,
    lines: u32,
    has_error: bool,
    current: usize,
}

impl Parser {
  pub fn new(tokens: Vec<Token>, lines: u32, has_error: bool) -> Parser {
    Parser {
      tokens,
      lines,
      has_error,
      current: 0,
    }
  }
  pub fn parse(&mut self, filename: &String) -> anyhow::Result<()> {
    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => bail!(Error::new(u8::MAX)),
    };

    *self = get_parser(file_contents.chars());
    return if self.has_error {
      bail!(Error::new(65))
      } else {
        Ok(())
      };
  }
}

fn get_parser(mut chars: Chars) -> Parser {
  let mut tokens = vec![];
  let mut line = 1;
  let mut has_error = false;
  while let Some(c) = chars.next() {
    let new_tokens = parse_token(c, &mut chars, &mut line, &mut has_error);
    tokens.extend(new_tokens);
  }
  tokens.push(Token::new(TokenType::EOF, "".to_string(), None));
  return Parser::new(tokens, line, has_error);
}

