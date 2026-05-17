#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub length: usize,
}

impl Span {
    pub fn new(start: usize, length: usize) -> Self {
        Self { start, length }
    }

    pub fn end(&self) -> usize {
        self.start + self.length
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
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
    EoF,
}

impl TokenKind {
    pub fn from_str(text: &str) -> Option<Self> {
        Some(match text {
            "let" => TokenKind::Let,
            "ind" => TokenKind::Inductive,
            "fn" => TokenKind::Fn,
            "Prop" => TokenKind::Prop,
            "Type" => TokenKind::Type,
            "unit" => TokenKind::Unit,
            "Self" => TokenKind::SelfType,
            "todo" => TokenKind::Todo,
            "lambda" => TokenKind::Lambda,
            "match" => TokenKind::Match,
            "with" => TokenKind::With,
            "forall" => TokenKind::Forall,
            _ => return None,
        })
    }
}
