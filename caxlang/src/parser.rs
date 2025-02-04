// External imports

// Internal imports
use crate::lexer::Token;

/// A literal type.
#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
    Expr(Box<Expr>)
}

/// An expression for the parser.
#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Grouping(Box<Expr>),
    Unary {
        op: Token,
        right: Box<Expr>
    },
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>
    }
}

/// Converts tokens into an AST.
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Parses the tokens into an AST.
    pub fn parse(&mut self) -> Vec<Box<Expr>> {
        // Create new AST
        let mut ast = Vec::new();

        ast.push(self.expression());

        // Return AST
        ast
    }

    /// Returns a new expression.
    fn expression(&mut self) -> Box<Expr> {
        self.equality()
    }

    /// Returns a new equality expression.
    fn equality(&mut self) -> Box<Expr> {
        let mut expr = self.comparison();

        while self.tmatch(vec![Token::NEqual, Token::DEqual]) {
            // Get operator
            let op = self.previous().clone();
            // Get current token
            let right = self.comparison();
            // Create binary expression with left, op, and right expressions
            expr = Box::new(Expr::Binary { left: expr, op, right })
        }

        expr
    }

    /// Returns a new comparison expression.
    fn comparison(&mut self) -> Box<Expr> {
        // Get current term
        let mut expr = self.term();

        // Add additional expressions, if necessary
        while self.tmatch(vec![Token::Less, Token::Greater, Token::LEqual, Token::GEqual]) {
            let op = self.previous().clone();
            let right = self.term();
            expr = Box::new(Expr::Binary { left: expr, op, right });
        }

        // Return new expr
        expr
    }

    /// Returns a new term expression.
    fn term(&mut self) -> Box<Expr> {
        // Get current term
        let mut expr = self.factor();

        // Add additional expressions, if necessary
        while self.tmatch(vec![Token::Minus, Token::Plus]) {
            let op = self.previous().clone();
            let right = self.factor();
            expr = Box::new(Expr::Binary { left: expr, op, right });
        }
 
        // Return new expr
        expr
    }

    /// Returns a new factor expression.
    fn factor(&mut self) -> Box<Expr> {
        // Get current unary
        let mut expr = self.unary();

        // Add additional expressions, if necessary
        while self.tmatch(vec![Token::Div, Token::Mult]) {
            let op = self.previous().clone();
            let right = self.unary();
            expr = Box::new(Expr::Binary { left: expr, op, right });
        }
 
        // Return new expr
        expr
    }

    /// Returns a new unary expression.
    fn unary(&mut self) -> Box<Expr> {
        // Parse '!' or '-', if necessary
        if self.tmatch(vec![Token::Bang, Token::Minus]) {
            let op = self.previous().clone();
            let right = self.unary();
            return Box::new(Expr::Unary { op, right });
        }
 
        // Return new primary
        self.primary()
    }

    /// Returns a new primary expression.
    fn primary(&mut self) -> Box<Expr> {
        if let Token::String(s) = self.peek().unwrap().clone() {
            self.advance();
            return Box::new(Expr::Literal(Literal::String(s)));
        }
        if let Token::Number(n) = self.peek().unwrap().clone() {
            self.advance();
            return Box::new(Expr::Literal(Literal::Number(n)));
        }
        else if self.tmatch(vec![Token::True]) {
            return Box::new(Expr::Literal(Literal::True));
        }
        else if self.tmatch(vec![Token::False]) {
            return Box::new(Expr::Literal(Literal::False));
        }
        else if self.tmatch(vec![Token::Nil]) {
            return Box::new(Expr::Literal(Literal::Nil));
        }
        else if self.tmatch(vec![Token::LParen]) {
            let expr = self.expression();
            self.consume(Token::RParen, "Expected ')' after expression!");
            return Box::new(Expr::Grouping(expr));
        }
        else {
            self.panic("Expected expression.");
            unimplemented!();
        }
    }

    /// Panics an detailed error, given a general message.
    fn panic(&self, msg: &'static str) {
        match self.peek() {
            Some(tk) => panic!("[line N]: at '{:?}', {}!", tk, msg),
            None => panic!("[line N]: {}!", msg)
        };
    }

    /// Trys to Consumes a token, panicing if the token does not exist.
    fn consume(&mut self, token: Token, panic_msg: &'static str) {
        if !self.tmatch(vec![token]) {
            panic!("{}", panic_msg);
        }
    }

    /// If the current token's value is equal to the one of the provided tokens,  
    /// advances and returns true. If one of the tokens does not match, returns false.
    fn tmatch(&mut self, tokens: Vec<Token>) -> bool {
        // Iterate over tokens and do stuff if they match current token
        for tk in tokens {
            if self.check(tk) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    /// Returns whether the current token matches a provided token value.  
    /// This does NOT consume any tokens, compared to `tmatch()`.
    /// 
    /// ## Panics
    /// This function panics if self.pos is greater than  
    /// or equal to self.tokens length
    fn check(&self, tk: Token) -> bool {
        self.peek().is_some() && *self.peek().unwrap() == tk
    }

    /// Returns the token at position `self.pos - 1`.
    /// 
    /// ## Panics
    /// This function panics if a token at `self.pos - 1` does not exist.
    fn previous(&self) -> &Token {
        self.tokens.get(self.pos - 1).unwrap()
    }

    /// Returns the current token at self.pos.  
    /// This does NOT modify the parser in any way.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    /// Returns the current token at self.pos
    /// and advances the position forward.
    fn advance(&mut self) -> Option<&Token> {
        // Create new token to store value
        let mut tk: Option<&Token> = None;

        // Assign to token if self.pos is in range of tokens
        if self.pos < self.tokens.len() {
            tk = self.tokens.get(self.pos);
            self.pos += 1;
        }

        // Return new token
        tk
    }
}

/// Returns the ast representation of code.
pub fn produce_ast(code: &'static str) -> Vec<Box<Expr>> {
    // Create new lexer
    let lexer = crate::lexer::tokenize(code);
    // Create parser from lexer
    let mut parser = Parser::new(lexer.map(|i| i.unwrap())
        .collect());

    // Return parsed ast
    parser.parse()
}

/// Prints ast tree as string.
pub fn print_ast(ast: &Vec<Box<Expr>>) {
    println!("{:?}", ast);
}

#[cfg(test)]
mod tests {
    // Use outside scope
    use super::*;

    /// Tests ast production for simple math operations.
    #[test]
    fn test_parser_simple_math_operations() {
        // Test addition
        print_ast(&produce_ast("1 + 2"));
        // Test subtraction
        print_ast(&produce_ast("1 - 2"));
        // Test multiplication
        print_ast(&produce_ast("1 * 2"));
        // Test division
        print_ast(&produce_ast("1 / 2"));
    }

    /// Tests ast production for correct unary value assignment.
    #[test]
    fn test_parser_unary() {
        // Ensure unary
        if let Expr::Binary {left, ..} = *(*produce_ast("-1 + -2").get(0).unwrap()).clone() {
            if let Expr::Unary {..} = (*left).clone() {
                
            } else {
                panic!("Unary expression expected. Got '{:?}'!", (*left).clone())
            }
        } else {
            panic!("Binary expression expected!");
        }
    }

    /// Tests ast production for complex math operations.
    #[test]
    fn test_parser_complex_math_operations() {
        // Test addition
        print_ast(&produce_ast("1 + 2 + 4"));
        // Test subtraction
        print_ast(&produce_ast("1 - 2 - 5"));
        // Test multiplication
        print_ast(&produce_ast("1 * 2 * 3"));
        // Test division
        print_ast(&produce_ast("1 / 2 / 7"));
    }
}
