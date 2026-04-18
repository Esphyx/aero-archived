use lexer::token::{Token, TokenKind};

#[derive(Debug)]
pub enum ParseErrorKind {
    UnexpectedToken {
        expected: TokenKind,
        found: TokenKind,
        message: String,
    },
    UnexpectedEof,
    Custom(String),
}

#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub position: usize,
    pub found: Option<Token>,
    pub context: Vec<&'static str>,
}
