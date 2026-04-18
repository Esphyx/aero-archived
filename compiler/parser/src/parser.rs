use lexer::token::{Token, TokenKind};

use crate::error::{ParseError, ParseErrorKind};

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

    pub fn error<T>(&self, kind: ParseErrorKind) -> Result<T, ParseError> {
        Err(ParseError {
            kind,
            position: self.current().position,
            found: Some(self.current().clone()),
            context: Vec::new(),
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

    pub fn expect_token(&mut self, expected: TokenKind) -> Result<(), ParseError> {
        let current = self.current();
        if current.kind == expected {
            self.advance();
            Ok(())
        } else {
            Err(ParseError {
                kind: ParseErrorKind::UnexpectedToken {
                    expected,
                    found: current.kind.clone(),
                    message: format!("Expected {:?}, found {:?}", expected, current.kind),
                },
                position: current.position,
                found: Some(current.clone()),
                context: Vec::new(),
            })
        }
    }


    // pub fn optional<F, T>(&mut self, parser: F) -> Result<Option<T>, ParseError>
    // where
    //     F: Fn(&mut Self) -> Result<T, ParseError>,
    // {
    //     // TODO
    //     let back_up = self.position;
    //     match parser(self) {
    //         Ok(result) => Ok(Some(result)),
    //         Err(e) => Err(e),
    //     }
    // }
}
