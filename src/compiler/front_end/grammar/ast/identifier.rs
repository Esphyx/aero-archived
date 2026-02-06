use std::fmt::Debug;

use super::{ParseError, Parser, TokenKind};

#[derive(PartialEq, Clone)]
pub struct SourceIdentifier {
    pub position: usize,
    pub length: usize,
    pub input: String,
}

impl SourceIdentifier {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let current_token = parser.current()?;

        if let TokenKind::Identifier = current_token.kind {
            let id = Self {
                position: current_token.position,
                length: current_token.length,
                input: parser.input.clone(),
            };
            parser.advance()?;
            Ok(id)
        } else {
            Err(ParseError::ExpectedIdentifier {
                found: current_token.clone(), // CLONE
                position: current_token.position,
            })
        }
    }

    pub fn get_name_str(&self) -> &str {
        &self.input[self.position..self.position + self.length]
    }
}

impl Debug for SourceIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.get_name_str())
    }
}
