use lexer::token::TokenKind;
use parser::parser::Parser;

use crate::{expression::Expression, identifier::Identifier, parameter::Parameter};

#[derive(Debug)]
pub struct Inductive {
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub typ: Expression,
    pub constructors: Vec<Constructor>,
}

impl Inductive {
    pub fn parse(parser: &mut Parser) -> Self {
        parser.expect_token(TokenKind::Inductive);

        let name = Identifier::parse(parser);

        let parameters = Parameter::parse_vec(parser);

        let typ = Expression::parse_type_specifier(parser);

        parser.expect_token(TokenKind::Assign);

        let mut constructors = Vec::new();
        while !(matches!(
            parser.peek_kind(),
            TokenKind::CloseBrace | TokenKind::SemiColon
        )) {
            if matches!(parser.peek_kind(), TokenKind::Pipe) {
                parser.advance();
            }
            constructors.push(Constructor::parse(parser));
        }

        parser.expect_token(TokenKind::SemiColon);

        Self {
            name,
            parameters,
            typ,
            constructors,
        }
    }
}

#[derive(Debug)]
pub struct Constructor {
    pub name: Identifier,
    pub typ: Expression,
}

impl Constructor {
    pub fn parse(parser: &mut Parser) -> Self {
        let name = Identifier::parse(parser);
        parser.expect_token(TokenKind::Colon);
        let typ = Expression::parse(parser);

        Self { name, typ }
    }
}
