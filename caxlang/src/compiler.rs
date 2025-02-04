// External imports
use std::any::Any;

// Internal imports
use crate::lexer::Token;
use crate::parser::{Expr, Literal};
use crate::vm::{Chunk, OpCode};

/// Interpreter that takes an AST and executes stuff based on it.
struct Compiler {
    /// The ast our compiler parses into bytecode.
    ast: Vec<Box<Expr>>,
}

impl Compiler {
    /// Returns a new compiler using the given ast.
    pub fn new(ast: Vec<Box<Expr>>) -> Self {
        Self { ast }
    }

    /// Parses an ast into a `Vec` of VM chunks.
    pub fn parse(&mut self) -> Vec<Chunk> {
        todo!()
    }

    /// Evaluates an expression and parses it to a chunk of bytecode.
    fn evaluate(&mut self, expr: &Box<Expr>) -> Chunk {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    // Use necessary items
    use super::*;
    use crate::lexer;
    use crate::parser;

    #[test]
    fn test_compiler_evaluate() {
        // Create ast
        let ast = parser::produce_ast("-1 + -2");
        // Print ast
        parser::print_ast(&ast);

        // Create compiler
        let mut compiler = Compiler::new(ast);
        // Get bytecode representation of ast
        // let bytecode = compiler.parse();

        // Print dissasembled bytecode for each chunk
        // for chunk in bytecode.iter() {
            // Do dissasembling and printing here!
        // }
    }
}
