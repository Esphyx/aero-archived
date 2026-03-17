use lexer::token::TokenKind;
use parser::parser::{ParseError, Parser};

use crate::{identifier::SourceIdentifier, term::SourceTerm};

#[derive(Debug)]
pub struct Parameter {
    pub name: SourceIdentifier,
    pub typ: SourceTerm,
}

impl Parameter {
    pub fn parse_vec(parser: &mut Parser) -> Result<Vec<Self>, ParseError> {
        let mut parameters = Vec::new();

        while matches!(parser.current()?.kind, TokenKind::OpenParen) {
            parser.advance()?;

            let name = SourceIdentifier::parse(parser)?;
            let typ = SourceTerm::parse_type_specifier(parser)?;

            parser.expect_token(TokenKind::CloseParen)?;
            parameters.push(Self { name, typ });
        }

        Ok(parameters)
    }
}
