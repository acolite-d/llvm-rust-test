use std::io::prelude::*;
use std::cell::{Cell, RefCell};

#[derive(Debug)]
pub enum Token {
    TokStart,
    TokEof,
    TokDef,
    TokExtern,
    TokIdentifier(String),
    TokNumber(f64),
    TokAscii(char),
}

pub struct Lexer {
    input: RefCell<Box<dyn BufRead>>,
    curr_token: RefCell<String>,
    curr_char: Cell<Option<char>>,
}

impl Lexer {
    pub fn new(input: Box<dyn BufRead>) -> Self {
        Self {
            input: RefCell::new(input),
            curr_token: RefCell::new(String::new()),
            curr_char: Cell::new(None),
        }
    }

    fn getchar(&self) {
        let next = self.input.borrow_mut()
            .as_mut()
            .bytes()
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as char);

        self.curr_char.set(next);
    }

    pub fn gettok(&self) -> Token {
        self.curr_token.borrow_mut().clear();

        loop {
            self.getchar();

            match self.curr_char.get() {
                Some(c) if c.is_whitespace() => continue,
                _ => break
            }
        }

        match self.curr_char.get() {
            Some(ch) if ch.is_alphabetic() => {
                while let Some(c) = self.curr_char.get() {
                    if c.is_whitespace() { break; }

                    self.curr_token.borrow_mut().push(c);
                    self.getchar();
                }

                match self.curr_token.borrow().as_str() {
                    "def" => Token::TokDef,
                    "extern" => Token::TokExtern,
                    _ => Token::TokIdentifier(
                        self.curr_token.borrow().clone()
                    ),
                }
            }

            Some(ch) if (ch.is_numeric() || ch == '.') => {
                while let Some(c) = self.curr_char.get() {
                    if c.is_whitespace() { break; }

                    self.curr_token.borrow_mut().push(c);
                    self.getchar();
                }

                let parsed_float: f64 = self.curr_token
                    .borrow()
                    .parse()
                    .unwrap();
                
                Token::TokNumber(parsed_float)
            }

            Some('#') => {
                loop {
                    match self.curr_char.get() {
                        Some('\n') | Some('\r') => { break; },
                        _ => { self.getchar() }
                    }
                }

                self.gettok()
            }

            Some(ascii_val) => Token::TokAscii(ascii_val),

            None => Token::TokEof,
        }
    }
}

pub struct TokenBuffer {
    lexer: Lexer,
    next: Option<Token>,
}

impl Lexer {
    pub fn into_iter(self) -> TokenBuffer {
        let first_token = self.gettok();
        TokenBuffer { lexer: self, next: Some(first_token) }
    }
}

impl Iterator for TokenBuffer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {

        let tok = self.next.replace(self.lexer.gettok());

        match tok {
            Some(Token::TokEof) => None,
            Some(tok) => Some(tok),
            None => None,
        }
    }
}