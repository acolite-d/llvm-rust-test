use std::io::prelude::*;

#[derive(Debug)]
pub enum Token {
    TokEof,
    TokDef,
    TokExtern,
    TokIdentifier(String),
    TokNumber(f64),
}

pub struct Lexer {
    input: Box<dyn BufRead>,
    curr_token: String,
    curr_char: Option<char>,
}

impl Lexer {
    pub fn new(input: Box<dyn BufRead>) -> Self {
        Self {
            input,
            curr_token: String::new(),
            curr_char: None,
        }
    }

    fn getchar(&mut self) {
        self.curr_char = self.input.as_mut()
            .bytes()
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as char)
    }

    pub fn gettok(&mut self) -> Token {
        self.curr_token.clear();

        loop {
            self.getchar();

            match self.curr_char {
                Some(c) if c.is_whitespace() => continue,
                _ => break
            }
        }

        match self.curr_char {
            Some(ch) if ch.is_alphabetic() => {
                while let Some(c) = self.curr_char {
                    if c.is_whitespace() { break; }

                    self.curr_token.push(c);
                    self.getchar();
                }

                match self.curr_token.as_str() {
                    "def" => Token::TokDef,
                    "extern" => Token::TokExtern,
                    _ => Token::TokIdentifier(
                        self.curr_token.clone()
                    ),
                }
            }

            Some(ch) if (ch.is_numeric() || ch == '.') => {
                while let Some(c) = self.curr_char {
                    if c.is_whitespace() { break; }

                    self.curr_token.push(c);
                    self.getchar();
                }

                let parsed_float: f64 = self.curr_token.parse().unwrap();
                Token::TokNumber(parsed_float)
            }

            Some('#') => {
                loop {
                    match self.curr_char {
                        Some('\n') | Some('\r') => { break; },
                        _ => { self.getchar() }
                    }
                }

                self.gettok()
            }

            Some(non_alpha_char) => {
                panic!(
                    "Err: failed to tokenize to due non-alphabetic character {:?}",
                    non_alpha_char
                )
            }

            None => Token::TokEof,
        }
    }
}