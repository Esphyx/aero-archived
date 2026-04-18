use std::fmt::Debug;

use lexer::token::TokenKind;
use parser::{
    error::{ParseError, ParseErrorKind},
    parser::Parser,
};

#[derive(PartialEq, Clone)]
pub struct Identifier {
    pub position: usize,
    pub length: usize,
    pub input: String,
}

impl Identifier {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let current_token = parser.current();

        if let TokenKind::Identifier = current_token.kind {
            let id = Self {
                position: current_token.position,
                length: current_token.length,
                input: parser.input.clone(),
            };
            parser.advance();
            Ok(id)
        } else {
            panic!("Expected identifier! {:?}", current_token.kind);
        }
    }

    pub fn get_name_str(&self) -> &str {
        &self.input[self.position..self.position + self.length]
    }
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.get_name_str())
    }
}
