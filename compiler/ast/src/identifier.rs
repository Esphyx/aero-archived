use lexer::token::TokenKind;
use parser::{error::ParseError, parser::Parser};

#[derive(PartialEq, Clone, Debug)]
pub struct Identifier {
    pub name: String,
}

impl From<String> for Identifier {
    fn from(name: String) -> Self {
        Self { name }
    }
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
            panic!("Expected identifier! found: {:?}", current_token.kind);
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
