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

impl  FunctionAST {
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

