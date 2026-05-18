use diagnostics::Span;
use lexer::token::TokenKind;

#[derive(Debug)]
pub enum ParseErrorKind {
    UnexpectedToken {
        expected: TokenKind,
        found: TokenKind,
        span: Span,
    },
    EOF,
}

#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
}
