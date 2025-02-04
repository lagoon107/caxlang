// External imports
use std::any::Any;

// Internal imports
use crate::lexer::Token;
use crate::parser::{Expr, Literal};

/// Interpreter that takes an AST and executes stuff based on it.
struct Interpreter {
    ast: Vec<Box<Expr>>
}

impl Interpreter {
    pub fn new(ast: Vec<Box<Expr>>) -> Self {
        Self { ast }
    }

    /// Evaluates all ast expressions.
    pub fn evaluate_all(&mut self) -> Vec<f64> {
        let mut nums = Vec::new();

        for expr in self.ast.clone().iter_mut() {
            nums.push(self.evaluate_num_expr(expr));
        }

        nums
    }

    /// Takes a number expression, evaluates it,  
    /// and returns the evaluated value.
    pub fn evaluate_num_expr(&mut self, expr: &mut Box<Expr>) -> f64 {
        match (**expr).clone() {
            Expr::Literal(l) => {
                if let Literal::Number(n) = l { n }
                else { panic!("Interpreter Error: Literal of type '{:?}' is not a valid number expression!", l) }
            },
            Expr::Unary {op, mut right} => {
                // Get value of right expression
                let right_val = self.evaluate_num_expr(&mut right);

                // Return based on operator
                if let Token::Bang = op {
                    -right_val
                } else {
                    right_val
                }
            },
            Expr::Grouping(mut g) => {
                self.evaluate_num_expr(&mut g)
            },
            Expr::Binary {mut left, op, mut right} => {
                let left_val = self.evaluate_num_expr(&mut left);
                let right_val = self.evaluate_num_expr(&mut right);

                match op {
                    Token::Plus => left_val + right_val,
                    Token::Minus => left_val - right_val,
                    Token::Mult => left_val * right_val,
                    Token::Div => left_val / right_val,
                    _ => panic!("Interpreter Error: Unsupported operator token in binary expression!")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Use necessary items
    use super::*;
    use crate::lexer;
    use crate::parser;

    #[test]
    fn test_interp_evaluate() {
        // Create ast
        let ast = parser::produce_ast("-1 + -2");
        // Print ast
        parser::print_ast(&ast);

        // Create interpreter
        let mut interp = Interpreter::new(ast);

        // Evaluate ast expr
        // println!("Evaluated: '{:?}'!", interp.evaluate_all())
    }
}
