use anyhow::{bail, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{BufRead, BufReader};

use crate::token::Token;

pub struct Tokenizer<R: BufRead> {
    reader: BufReader<R>,
    current_line: String,
    line_number: usize,
}

impl<R: BufRead> Tokenizer<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
            current_line: String::new(),
            line_number: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while self.read_line()? {
            while !self.current_line.is_empty() {
                if self.current_line.starts_with(char::is_whitespace) {
                    Self::trim_start_inplace(&mut self.current_line);
                } else {
                    let token = self.next_token()?;
                    tokens.push(token);
                }
            }
        }

        Ok(tokens)
    }

    fn read_line(&mut self) -> Result<bool> {
        self.current_line.clear();
        self.line_number += 1;
        match self.reader.read_line(&mut self.current_line) {
            Ok(0) => Ok(false), // End of file
            Ok(_) => {
                self.current_line = self.current_line.trim_end().to_string(); // Remove trailing newline
                Ok(true)
            }
            Err(e) => bail!("Error reading line {}: {}", self.line_number, e),
        }
    }

    fn next_token(&mut self) -> Result<Token> {
        if self.current_line.is_empty() {
            bail!("Unexpected end of line {}", self.line_number);
        }

        if let Some((token, len)) = self.match_identifier().or_else(|| self.match_number()) {
            self.current_line.replace_range(..len, "");
            return Ok(token);
        }

        let token = match self.current_line.chars().next().unwrap() {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            ';' => Token::Semicolon,
            _ => bail!("Invalid token at line {}", self.line_number),
        };
        self.current_line.replace_range(..1, "");
        Ok(token)
    }

    fn match_identifier(&self) -> Option<(Token, usize)> {
        static IDENTIFIER_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^[a-zA-Z_]\w*\b").unwrap());
        if let Some(mat) = IDENTIFIER_REGEX.find(&self.current_line) {
            let matched = mat.as_str();
            Some(match matched {
                "int" => (Token::Int, 3),
                "void" => (Token::Void, 4),
                "return" => (Token::Return, 6),
                _ => (Token::Identifier(matched.to_string()), matched.len()),
            })
        } else {
            None
        }
    }

    fn match_number(&self) -> Option<(Token, usize)> {
        static NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9]+\b").unwrap());
        if let Some(mat) = NUMBER_REGEX.find(&self.current_line) {
            let matched = mat.as_str();
            let number = matched.parse().unwrap();
            Some((Token::Number(number), matched.len()))
        } else {
            None
        }
    }

    fn trim_start_inplace(s: &mut String) {
        let trimmed = s.trim_start();
        s.replace_range(..(s.len() - trimmed.len()), "");
    }
}



