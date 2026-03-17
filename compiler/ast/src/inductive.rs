use lexer::token::TokenKind;
use parser::parser::{ParseError, Parser};

use crate::{identifier::SourceIdentifier, parameter::Parameter, term::SourceTerm};

#[derive(Debug)]
pub struct SourceInductive {
    pub name: SourceIdentifier,
    pub parameters: Vec<Parameter>,
    pub typ: SourceTerm,
    pub constructors: Vec<SourceConstructor>,
}

impl SourceInductive {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect_token(TokenKind::Inductive)?;

        let name = SourceIdentifier::parse(parser)?;

        // PARAMETERS
        let mut parameters = Vec::new();
        while matches!(parser.current()?.kind, TokenKind::OpenParen) {
            parser.advance()?;
            let parameter_name = SourceIdentifier::parse(parser)?;
            parser.expect_token(TokenKind::Colon)?;
            let typ = SourceTerm::parse(parser)?;
            parser.expect_token(TokenKind::CloseParen)?;
            parameters.push(Parameter {
                name: parameter_name,
                typ,
            });
        }

        let typ = SourceTerm::parse_type_specifier(parser)?;

        // body
        parser.expect_token(TokenKind::Assign)?;

        // constructors
        let mut constructors = Vec::new();
        while !(matches!(
            parser.current()?.kind,
            TokenKind::CloseBrace | TokenKind::SemiColon
        )) {
            if matches!(parser.current()?.kind, TokenKind::Pipe) {
                parser.advance()?;
            }
            constructors.push(SourceConstructor::parse(parser, &parameters)?);
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
pub struct SourceConstructor {
    pub name: SourceIdentifier,
    pub typ: SourceTerm,
}

impl SourceConstructor {
    pub fn parse(parser: &mut Parser, parameters: &Vec<Parameter>) -> Result<Self, ParseError> {
        let name = SourceIdentifier::parse(parser)?;
        parser.expect_token(TokenKind::Colon)?;
        let typ = SourceTerm::parse(parser)?;

        Ok(Self { name, typ })
    }
}
