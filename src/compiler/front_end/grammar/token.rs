#[derive(Debug, Clone)]
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

#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    // DYNAMIC LENGTH
    Identifier, NumberLiteral, StringLiteral, Comment,
    // KEYWORDS
    Todo, Total, Partial, Inductive, Fn, External,
    // PRIMITIVES
    Unit, Prop, Type, U8, WriteU8, ReadU8, AddU8,
    // OPERATORS
    Plus, Minus, Star, Slash, Pipe, Greater, Less, ShiftRight, ShiftLeft,
    // DELIMITERS
    Colon, SemiColon, Period, Comma, Arrow,
    // BRACES
    OpenBracket, CloseBracket, OpenParen, CloseParen, OpenBrace, CloseBrace,
    // UNUSED
    Virtual, Entry, U32, Match, Assign, Let, Do, While, Loop, Break, Continue, Snail,
    // END OF FILE 
    EoF,
}

impl TokenKind {
    pub fn from_str(text: &str) -> Option<Self> {
        Some(match text {
            "let" => TokenKind::Let,
            "ind" => TokenKind::Inductive,
            "=>" => TokenKind::Arrow,
            _ => return None,
        })
    }
}
