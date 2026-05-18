use lexer::token::TokenKind;
use parser::parser::Parser;

use crate::{expression::Expression, identifier::Identifier};

#[derive(Debug)]
pub struct Parameter {
    pub name: Identifier,
    pub typ: Expression,
}

impl Parameter {
    pub fn parse_vec(parser: &mut Parser) -> Vec<Self> {
        let mut parameters = Vec::new();

        while matches!(parser.peek_kind(), TokenKind::OpenParen) {
            parser.advance();

            let first_name = Identifier::parse(parser);

            let mut names = vec![first_name];
            while matches!(parser.peek_kind(), TokenKind::Comma) {
                parser.advance();
                let name = Identifier::parse(parser);
                names.push(name);
            }

            let typ = Expression::parse_type_specifier(parser);

            parser.expect_token(TokenKind::CloseParen);

            for name in names {
                parameters.push(Self {
                    name,
                    typ: typ.clone(),
                });
            }
        }

        parameters
    }
}
