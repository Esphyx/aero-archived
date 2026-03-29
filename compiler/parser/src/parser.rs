use std::fmt::Display;

use lexer::{
    lexer::LexicalError,
    token::{Token, TokenKind},
};

#[allow(unused)]
#[derive(Debug)]
pub enum ParseError {
    ExpectedToken {
        expected: Vec<TokenKind>,
        found: Token,
        position: usize,
    },
    ExpectedIdentifier {
        found: Token,
        position: usize,
    },
    ExpectedExpression {
        found: TokenKind,
        position: usize,
    },
    LexicalError(LexicalError),
    OutOfTokens,
}

impl ParseError {
    pub fn with_source(&self, source: String) -> String {
        match self {
            ParseError::ExpectedToken {
                expected,
                found,
                position,
            } => {
                let snippet = &source[*position..(*position + found.length).max(source.len())];
                format!(
                    "Expected {:?}, found '{:?}' at {}: '{}'",
                    expected, found.kind, position, snippet
                )
            }
            ParseError::ExpectedIdentifier { found, position } => {
                let snippet = &source[*position..(*position + found.length).max(source.len())];
                format!(
                    "Expected identifier, found '{:?}' at {}: '{}'",
                    found.kind, position, snippet
                )
            }
            _ => format!("{:?}", self),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ParseError {}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    pub input: String,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, input: String) -> Result<Self, ParseError> {
        Ok(Self {
            tokens,
            position: 0,
            input,
        })
    }

    pub fn current(&self) -> &Token {
        self.tokens.get(self.position).expect("Out of tokens")
    }

    pub fn advance(&mut self) {
        loop {
            self.position += 1;

            if !matches!(self.current().kind, TokenKind::Comment) {
                break;
            }
        }
    }

    pub fn expect_token(&mut self, expected: TokenKind) {
        let current_token = self.current();
        if current_token.kind == expected {
            self.advance();
        } else {
            panic!("Expected token: {:?}, found: {:?}", expected, (*current_token).clone());
            // Err(ParseError::ExpectedToken {
            //     expected: vec![expected],
            //     found: (*current_token).clone(), // CLONE
            //     position: current_token.position,
            // })
        }
    }

    pub fn optional<F, T>(&mut self, parser: F) -> Result<Option<T>, ParseError>
    where
        F: Fn(&mut Self) -> Result<T, ParseError>,
    {
        let back_up = self.position;
        match parser(self) {
            Ok(result) => Ok(Some(result)),
            Err(ParseError::ExpectedExpression { .. })
            | Err(ParseError::ExpectedIdentifier { .. })
            | Err(ParseError::ExpectedToken { .. }) => {
                self.position = back_up;
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }
}
