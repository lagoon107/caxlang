// External imports
use logos::Logos;

// Internal imports

use std::num::ParseIntError;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    InvalidInteger(String),
    #[default]
    NonAsciiCharacter,
}

/// Error type returned by calling `lex.slice().parse()` to u8.
impl From<ParseIntError> for LexingError {
    fn from(err: ParseIntError) -> Self {
        use std::num::IntErrorKind::*;
        match err.kind() {
            PosOverflow | NegOverflow => LexingError::InvalidInteger("overflow error".to_owned()),
            _ => LexingError::InvalidInteger("other error".to_owned()),
        }
    }
}

/// The individual tokens our lexer constructs.
#[derive(Debug, Logos, Clone, PartialEq)]
#[logos(error = LexingError)]
#[logos(skip r"[ \t\n]")]
pub enum Token {
    // Number
    #[regex(r"(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),

    // String
    #[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt/]|u[a-fA-F0-9]{4}))*""#, |lex| lex.slice().to_owned().replace("\"", ""))]
    String(String),

    // Ident
    #[regex(r"[A-Z_a-z]+", |lex| lex.slice().to_string())]
    Ident(String),

    // Math operators
    #[regex(r"\+")]
    Plus,
    #[regex(r"-")]
    Minus,
    #[regex(r"\*")]
    Mult,
    #[regex(r"/")]
    Div,

    // Equality
    #[regex(r"true")]
    True,
    #[regex(r"false")]
    False,
    #[regex(r"=")]
    Equal,
    #[regex(r"!")]
    Bang,
    #[regex(r"!=")]
    NEqual,
    #[regex(r"<")]
    Less,
    #[regex(r">")]
    Greater,
    /// Greater than or equal.
    #[regex(r">=")]
    GEqual,
    /// Less than or equal.
    #[regex(r"<=")]
    LEqual,
    /// Double Equal.
    #[regex("==")]
    DEqual,

    // Grouping
    #[regex(r"\(")]
    LParen,
    #[regex(r"\)")]
    RParen,

    // Other
    #[regex(r"nil")]
    Nil
}

/// Tokenizes code into a series of tokens.
pub fn tokenize(code: &'static str) -> impl Iterator<Item = Result<Token, LexingError>> {
    Token::lexer(code)
}

#[cfg(test)]
mod tests {
    // Use outside scope
    use super::*;

    /// Tests the `tokenize()` function with basic input.
    #[test]
    fn test_tokenize() {
        // Try tokenizing simple string and printing tokens
        for tk in tokenize(
            "123.42 (23.43) * 123.43 sd + \"Hello\""
        ) {
            println!("{:?}", match tk {
                Ok(tk) => tk,
                Err(e) => panic!("Lexer Error: '{:?}'!", e)
            });
        }
    }
}
