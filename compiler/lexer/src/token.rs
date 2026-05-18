use diagnostics::Span;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub lexeme: Option<String>,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self {
            kind,
            span,
            lexeme: None,
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
    Match, With, Forall, SelfType,
    // OPERATORS
    Assign,
    // BUILTIN
    Prop, Type,
    // DELIMITERS
    Colon, SemiColon, Comma, Arrow, Pipe, FatArrow,
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
            "Self" => TokenKind::SelfType,
            "lambda" => TokenKind::Lambda,
            "match" => TokenKind::Match,
            "with" => TokenKind::With,
            "forall" => TokenKind::Forall,
            _ => return None,
        })
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            TokenKind::Let => "let",
            TokenKind::Identifier => "identifier",
            TokenKind::Comment => "comment",
            TokenKind::Inductive => "inductive",
            TokenKind::Fn => "fn",
            TokenKind::Lambda => "lambda",
            TokenKind::Match => "match",
            TokenKind::With => "with",
            TokenKind::Forall => "forall",
            TokenKind::SelfType => "Self",
            TokenKind::Assign => ":=",
            TokenKind::Prop => "Prop",
            TokenKind::Type => "Type",
            TokenKind::Colon => ":",
            TokenKind::SemiColon => ";",
            TokenKind::Comma => ",",
            TokenKind::Arrow => "->",
            TokenKind::Pipe => "|",
            TokenKind::FatArrow => "=>",
            TokenKind::OpenBracket => "[",
            TokenKind::CloseBracket => "]",
            TokenKind::OpenParen => "(",
            TokenKind::CloseParen => ")",
            TokenKind::OpenBrace => "{",
            TokenKind::CloseBrace => "}",
            TokenKind::EoF => "EOF",
        }
    }
}
