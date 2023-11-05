use std::{error::Error, fmt};
use std::collections::HashMap;
use std::cell::{RefCell};

use super::lexer::{Lexer, Token};

pub trait ASTExpr {  }

struct NumberExprAST(f64);

impl NumberExprAST {
    pub fn new(number: f64) -> Self {
        Self(number)
    }
}

struct VariableExprAST(String);

impl VariableExprAST {
    pub fn new(var: String) -> Self {
        Self(var)
    }
}

struct BinaryExprAST {
    operator: char,
    lhs_operand: Box<dyn ASTExpr>,
    rhs_operand: Box<dyn ASTExpr>,
}

impl BinaryExprAST {
    pub fn new(
        operator: char,
        lhs_operand: Box<dyn ASTExpr>,
        rhs_operand: Box<dyn ASTExpr>,
    ) -> Self {
        Self { operator, lhs_operand, rhs_operand }
    }
}

struct CallExprAST {
    callee: String,
    args: Vec<Box<dyn ASTExpr>>,
}

impl CallExprAST {
    pub fn new(callee: String, args: Vec<Box<dyn ASTExpr>>) -> Self {
        Self { callee, args }
    }
}

struct PrototypeAST {
    name: String,
    args: Vec<String>,
}

impl PrototypeAST {
    pub fn new(name: String, args: Vec<String>) -> Self {
        Self { name, args }
    }
}

struct FunctionAST {
    proto: Box<PrototypeAST>,
    body: Box<dyn ASTExpr>,
}

impl FunctionAST {
    pub fn new(proto: Box<PrototypeAST>, body: Box<dyn ASTExpr>) -> Self {
        Self { proto, body }
    }
}

impl ASTExpr for NumberExprAST {  }

impl ASTExpr for VariableExprAST {  }

impl ASTExpr for BinaryExprAST {  }

impl ASTExpr for CallExprAST {  }

impl ASTExpr for PrototypeAST {  }

impl ASTExpr for FunctionAST {  }


// Implementing your own Errors for Results requires defining your type
// then making sure that type implements Error: Debug + Display

#[derive(Debug)]
pub struct ParseError(&'static str);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.0)
    }
}

impl Error for ParseError {  }

pub struct Parser<'lexer> {
    binop_precedence: HashMap<char, i32>,
    lexer: &'lexer Lexer,
    curr_token: RefCell<Option<Token>>
}


impl<'lexer> Parser<'lexer> {
    pub fn new(
        binop_precedence: HashMap<char, i32>, 
        lexer: &'lexer Lexer,
    ) -> Self {
        Self { 
            binop_precedence, 
            lexer,
            curr_token: RefCell::new(None)
        }
    }

    fn get_next_token(&self) {
        self.curr_token.borrow_mut().replace(self.lexer.gettok());
    }

    fn get_operator_precedence(&self, op: char) -> Result<i32, ParseError> {
        self.binop_precedence
            .get(&op)
            .ok_or(ParseError("Unrecognized binary operator!"))
            .map(|preced| *preced)
    }
        

    fn parse_num_expr(&self, num: f64) -> Box<NumberExprAST> {
        Box::new(NumberExprAST::new(num))
    }

    fn parse_identifier_expr(&self, id: String) -> Result<Box<dyn ASTExpr>, ParseError>  {
        self.get_next_token();

        if let Some(Token::TokAscii('(')) = *self.curr_token.borrow() {

            self.get_next_token();
            let mut args_vec: Vec<Box<dyn ASTExpr>> = Vec::new();

            loop {
                if let Some(Token::TokAscii(')')) = *self.curr_token.borrow() {
                    break;
                }

                let expr = self.parse_primary()?;
                args_vec.push(expr);

                match *self.curr_token.borrow() {
                    Some(Token::TokAscii(')')) => break,
                    Some(Token::TokAscii(',')) => { },
                    _ => { return Err(ParseError("Expected ')' or ',' in argument list")); }
                }

                self.get_next_token();
            }
            
            Ok(Box::new(CallExprAST::new(id, args_vec)))

        } else {
            Ok(Box::new(VariableExprAST::new(id)))
        }
    }

    fn parse_paren_expr(&self) -> Result<Box<dyn ASTExpr>, ParseError> {
        self.get_next_token();
        let expr = self.parse_primary();

        if let Some(Token::TokAscii(')')) = *self.curr_token.borrow() {
            self.get_next_token();
            expr
        } else {
            Err(ParseError("Expected ')'"))
        }
    }

    fn parse_primary(&self) -> Result<Box<dyn ASTExpr>, ParseError> {
        match *self.curr_token.borrow() {
            Some(Token::TokIdentifier(ref id)) => {
                self.parse_identifier_expr(id.clone())
            },

            Some(Token::TokNumber(num)) => {
                Ok(self.parse_num_expr(num))
            },

            Some(Token::TokAscii('(')) => {
                self.parse_paren_expr()
            }

            _ => Err(ParseError("Failed at primary expression!"))
        }
    }


}

pub fn main_loop() {

}