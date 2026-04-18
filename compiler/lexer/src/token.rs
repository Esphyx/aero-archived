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
    Identifier, Comment,
    // KEYWORDS
    Let, Inductive, Fn, Lambda,
    Todo, Match, With, Forall, SelfType,
    // OPERATORS
    Assign,
    // BUILTIN
    Unit, Prop, Type,
    // DELIMITERS
    Colon, SemiColon,Comma, Arrow, Pipe, FatArrow, Wildcard,
    // BRACES
    OpenBracket, CloseBracket, OpenParen, CloseParen, OpenBrace, CloseBrace,
    // UNUSED
    // Virtual, Entry, Do, While, Loop, Break, Continue, Snail, External, Total, Partial,
    // Plus, Minus, Star, Slash, Greater, Less, ShiftRight, ShiftLeft, Period, 
    // END OF FILE 
    EoF,
}

impl TokenKind {
    pub fn from_str(text: &str) -> Option<Self> {
        Some(match text {
            "let" => TokenKind::Let,
            "ind" => TokenKind::Inductive,
            "fn" => TokenKind::Fn,
            "=>" => TokenKind::Arrow,
            _ => return None,
        })
    }
}
