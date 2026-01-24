#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub position: usize,
    pub length: usize,
}

impl Token {
    pub fn new(kind: TokenKind, position: usize, length: usize) -> Self {
        Self {
            kind,
            position,
            length,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    Identifier,
    NumberLiteral,
    StringLiteral,
    Comment,
    // KEYWORDS
    Todo,
    Total,
    Partial,
    Inductive,
    Fn,
    External,
    // PRIMITIVES
    Prop,
    Type,
    U8,
    // OPERATORS
    Plus,
    Minus,
    Star,
    Slash,
    Pipe,
    Greater,
    Less,
    ShiftRight,
    ShiftLeft,
    // DELIMITERS
    Colon,
    SemiColon,
    Period,
    Comma,
    Arrow,
    // BRACES
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    // UNUSED
    Virtual,
    Entry,
    U32,
    Match,
    Assign,
    Let,
    Do,
    While,
    Loop,
    Break,
    Continue,
    Snail,
    // EoF
    EndOfFile,
}
