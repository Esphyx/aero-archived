use lexer::token::TokenKind;
use parser::{error::ParseError, parser::Parser};

use crate::{expression::Expression, identifier::Identifier, parameter::Parameter};

#[derive(Debug)]
pub struct Inductive {
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub typ: Expression,
    pub constructors: Vec<Constructor>,
}

impl Inductive {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect_token(TokenKind::Inductive)?;

        let name = Identifier::parse(parser)?;

        let parameters = Parameter::parse_vec(parser)?;

        let typ = Expression::parse_type_specifier(parser)?;

        parser.expect_token(TokenKind::Assign)?;

        let mut constructors = Vec::new();
        while !(matches!(
            parser.current().kind,
            TokenKind::CloseBrace | TokenKind::SemiColon
        )) {
            if matches!(parser.current().kind, TokenKind::Pipe) {
                parser.advance();
            }
            constructors.push(Constructor::parse(parser)?);
        }

        parser.expect_token(TokenKind::SemiColon)?;

        Ok(Self {
            name,
            parameters,
            typ,
            constructors,
        })
    }
}

#[derive(Debug)]
pub struct Constructor {
    pub name: Identifier,
    pub typ: Expression,
}

impl Constructor {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let name = Identifier::parse(parser)?;
        parser.expect_token(TokenKind::Colon)?;
        let typ = Expression::parse(parser)?;

        Ok(Self { name, typ })
    }
}
