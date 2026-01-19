#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    position: usize,
    length: usize,
}

pub struct GrammarRule {
    pub kind: TokenKind,
    pub matcher: Box<dyn Fn(&str) -> usize>,
}

impl GrammarRule {
    pub fn new<F>(kind: TokenKind, matcher: F) -> Self
    where
        F: Fn(&str) -> usize + 'static,
    {
        Self {
            kind,
            matcher: Box::new(matcher),
        }
    }

    pub fn from(kind: TokenKind, literals: &'static [&'static str]) -> Self {
        Self {
            kind,
            matcher: Box::new(move |slice| {
                for literal in literals {
                    if slice.starts_with(literal) {
                        return literal.len();
                    }
                }

                0
            }),
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
    Type,
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
    U8,
    // EoF
    EndOfFile,
}

pub struct ParseError {
    pub position: usize,
    pub found: Option<TokenKind>,
}

pub struct GrammarParser<'a> {
    input: &'a str,
    rules: Vec<GrammarRule>,
}

impl<'a> GrammarParser<'a> {
    pub fn new(input: &'a str) -> Self {
        let rules = vec![
            GrammarRule::from(TokenKind::Inductive, &["inductive"]),
            GrammarRule::from(TokenKind::Arrow, &["->"]),
            GrammarRule::from(TokenKind::Minus, &["-"]),
            GrammarRule::from(TokenKind::Greater, &[">"]),
            GrammarRule::from(TokenKind::Less, &["<"]),
        ];
        Self { input, rules }
    }

    pub fn tokens_at(&self, position: usize) -> Vec<Token> {
        let slice = &self.input[position..];
        let mut tokens = Vec::new();

        for GrammarRule { kind, matcher } in &self.rules {
            let length = matcher(slice);
            if length > 0 {
                tokens.push(Token {
                    kind: *kind,
                    position,
                    length,
                });
            }
        }

        tokens
    }
}
