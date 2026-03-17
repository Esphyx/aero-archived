use lexer::token::TokenKind;
use parser::parser::{ParseError, Parser};

use crate::{identifier::SourceIdentifier, parameter::Parameter, term::SourceTerm};

#[derive(Debug)]
pub struct SourceFunction {
    pub name: SourceIdentifier,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<SourceTerm>,
    pub body: SourceTerm,
}

impl SourceFunction {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect_token(TokenKind::Fn)?;
        let name = SourceIdentifier::parse(parser)?;

        let parameters = Parameter::parse_vec(parser)?;

        let return_type = parser.optional(SourceTerm::parse_type_specifier)?;

        parser.expect_token(TokenKind::OpenBrace)?;
        let body = SourceTerm::parse(parser)?;
        parser.expect_token(TokenKind::CloseBrace)?;

        Ok(Self {
            name,
            parameters,
            return_type,
            body,
        })
    }
}
