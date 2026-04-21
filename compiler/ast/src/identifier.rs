use std::fmt::Debug;

use lexer::token::TokenKind;
use parser::{error::ParseError, parser::Parser};

#[derive(PartialEq, Clone)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let current_token = parser.current();

        if let TokenKind::Identifier = current_token.kind {
            let name = parser.input
                [current_token.position..current_token.position + current_token.length]
                .to_string();

            parser.advance();
            Ok(Self { name })
        } else {
            panic!("Expected identifier! {:?}", current_token.kind);
        }
    }

    pub fn dummy() -> Self {
        Self {
            name: "_".to_string(),
        }
    }

    pub fn get_name_str(&self) -> &str {
        &self.name
    }
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.get_name_str())
    }
}
