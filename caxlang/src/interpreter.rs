// External imports

// Internal imports
use crate::lexer::Token;
use crate::parser::{Expr, Literal};

/// A runtime bool type.
#[derive(Debug)]
pub enum RuntimeBool {
    True,
    False
}

/// A runtime value.
#[derive(Debug)]
pub enum RuntimeVal {
    String(String),
    Number(f64),
    Bool(RuntimeBool),
    Nil
}

/// An error that can be returned from the interpreter.
#[derive(Debug, thiserror::Error)]
pub enum InterpError {
    #[error("literal is of type 'Expr'")]
    LiteralIsExpr,
    #[error("Unary operator of type {0:?} not valid on {1:?}")]
    InvalidUnaryOperator(Token, RuntimeVal),
    #[error("Unary operator of type {0:?} not supported on non-numbers")]
    UnaryOperatorOnNonNumber(Token),
    #[error("Binary operator of type {0:?} not valid between {1:?} and {2:?}")]
    InvalidBinaryOperator(Token, RuntimeVal, RuntimeVal),
    #[error("Binary operator of type {0:?} not valid between non numbers {1:?} and {2:?}")]
    BinaryOperatorOnNonNumber(Token, RuntimeVal, RuntimeVal)
}

/// Interpreter that takes an AST and executes stuff based on it.
pub struct Interpreter {
    /// The ast our compiler parses into bytecode.
    ast: Vec<Box<Expr>>,
}

impl Interpreter {
    /// Returns a new compiler using the given ast.
    pub fn new(ast: Vec<Box<Expr>>) -> Self {
        Self { ast }
    }

    /// Parses the ast.  
    /// The return value should only be used for debugging or error-checking.
    pub fn parse(self) -> Vec<Result<RuntimeVal, InterpError>> {
        // Create vec to store runtime vals
        let mut runtime_vals = Vec::new();

        for expr in self.ast {
            runtime_vals.push(Self::evaluate(expr))
        }

        // Return runtime vals with no errors
        runtime_vals
    }

    /// Evaluates an Expr ast node and returns a runtime value.
    fn evaluate(expr: Box<Expr>) -> Result<RuntimeVal, InterpError> {
        match *expr {
            // PARSING FOR MATH EXPRESSIONS
            Expr::Literal(l) => {
                match l {
                    Literal::String(s) => Ok(RuntimeVal::String(s)),
                    Literal::Number(n) => Ok(RuntimeVal::Number(n)),
                    Literal::Nil => Ok(RuntimeVal::Nil),
                    Literal::Expr(_) => Err(InterpError::LiteralIsExpr),
                    Literal::True => Ok(RuntimeVal::Bool(RuntimeBool::True)),
                    Literal::False => Ok(RuntimeVal::Bool(RuntimeBool::False))
                }
            },
            Expr::Unary {op, ref right} => {
                // Evaluate right expression
                let right_eval = Self::evaluate(right.clone())?;

                match op {
                    Token::Minus => {
                        // Return negative number if right_eval is num
                        if let RuntimeVal::Number(n) = right_eval {
                            Ok(RuntimeVal::Number(-n))
                        } else {
                            Err(InterpError::UnaryOperatorOnNonNumber(op))
                        }
                    },
                    _ => {
                        Err(InterpError::InvalidUnaryOperator(op, right_eval))
                    }
                }
            }
            Expr::Binary {left, op, right} => {
                // Get evaluated right and left expressions
                let left_eval = Self::evaluate(left)?;
                let right_eval = Self::evaluate(right)?;

                // Return expression based on operator
                match op {
                    Token::Plus => {
                        // Try to get number of both values
                        if let RuntimeVal::Number(left_num) = left_eval {
                            if let RuntimeVal::Number(right_num) = right_eval {
                                Ok(RuntimeVal::Number(left_num + right_num))
                            } else {
                                Err(InterpError::BinaryOperatorOnNonNumber(op, left_eval, right_eval))
                            }
                        } else {
                            Err(InterpError::BinaryOperatorOnNonNumber(op, left_eval, right_eval))
                        }
                    }
                    Token::Minus => {
                        // Try to get number of both values
                        if let RuntimeVal::Number(left_num) = left_eval {
                            if let RuntimeVal::Number(right_num) = right_eval {
                                Ok(RuntimeVal::Number(left_num - right_num))
                            } else {
                                Err(InterpError::BinaryOperatorOnNonNumber(op, left_eval, right_eval))
                            }
                        } else {
                            Err(InterpError::BinaryOperatorOnNonNumber(op, left_eval, right_eval))
                        }
                    }
                    tk => Err(InterpError::InvalidBinaryOperator(tk, left_eval, right_eval))
                }
            }
            Expr::Grouping(g) => {
                Ok(Self::evaluate(g)?)
            }
        }
    }
}

/// Takes a piece of code and lexes it, parses it, and interpretes it, displaying its result.
pub fn interp_code(code: &'static str) -> Result<(), InterpError> {
    // Use statements
    use super::parser;

    // Produce ast
    let ast = parser::produce_ast(code);

    // Create interpreter
    let interp = Interpreter::new(ast);

    // Loop through generated runtime values
    for runtime_val in interp.parse() {
        match runtime_val {
            Ok(r) => println!("\t{:?}", r),
            Err(e) => return Err(e)
        }
    }

    // Return no errors
    Ok(())
}

#[cfg(test)]
mod tests {
    // Use necessary items
    use super::*;

    /// Test result of adding positive numbers.
    #[test]
    fn test_interp_positive_add() {
        println!("\n-------NEGATIVE ADD RESULT-------");
        interp_code("1 + 2").unwrap();
    }

    /// Test result of subtracting positive numbers.
    #[test]
    fn test_interp_negative_add() {
        println!("\n-------NEGATIVE ADD RESULT-------");
        interp_code("1 - 2").unwrap();
    }
}
