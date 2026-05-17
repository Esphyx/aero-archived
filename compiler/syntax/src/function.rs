use lexer::token::TokenKind;
use parser::{error::ParseError, parser::Parser};

use crate::{identifier::Identifier, parameter::Parameter, expression::Expression};

#[derive(Debug)]
pub struct Function {
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub return_type: Expression,
    pub body: Expression,
}

impl Function {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect_token(TokenKind::Fn)?;
        let name = Identifier::parse(parser)?;

        let parameters = Parameter::parse_vec(parser)?;

        let return_type = Expression::parse_type_specifier(parser)?;

        parser.expect_token(TokenKind::OpenBrace)?;
        let body = Expression::parse(parser)?;
        parser.expect_token(TokenKind::CloseBrace)?;

        Ok(Self {
            name,
            parameters,
            return_type,
            body,
        })
    }
}
