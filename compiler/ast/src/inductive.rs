use lexer::token::TokenKind;
use parser::parser::{ParseError, Parser};

use crate::{identifier::Identifier, parameter::Parameter, expression::Expression};

#[derive(Debug)]
pub struct Inductive {
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub typ: Expression,
    pub constructors: Vec<Constructor>,
}

impl Inductive {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect_token(TokenKind::Inductive);

        let name = Identifier::parse(parser)?;

        // PARAMETERS
        let mut parameters = Vec::new();
        while matches!(parser.current().kind, TokenKind::OpenParen) {
            parser.advance();
            let parameter_name = Identifier::parse(parser)?;
            parser.expect_token(TokenKind::Colon);
            let typ = Expression::parse(parser)?;
            parser.expect_token(TokenKind::CloseParen);
            parameters.push(Parameter {
                name: parameter_name,
                typ,
            });
        }

        let typ = Expression::parse_type_specifier(parser)?;

        // body
        parser.expect_token(TokenKind::Assign);

        // constructors
        let mut constructors = Vec::new();
        while !(matches!(
            parser.current().kind,
            TokenKind::CloseBrace | TokenKind::SemiColon
        )) {
            if matches!(parser.current().kind, TokenKind::Pipe) {
                parser.advance();
            }
            constructors.push(Constructor::parse(parser, &parameters)?);
        }

        parser.expect_token(TokenKind::SemiColon);

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
    pub fn parse(parser: &mut Parser, parameters: &Vec<Parameter>) -> Result<Self, ParseError> {
        let name = Identifier::parse(parser)?;
        parser.expect_token(TokenKind::Colon);
        let typ = Expression::parse(parser)?;

        Ok(Self { name, typ })
    }
}
