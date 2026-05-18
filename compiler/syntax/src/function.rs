use lexer::token::TokenKind;
use parser::parser::Parser;

use crate::{expression::Expression, identifier::Identifier, parameter::Parameter};

#[derive(Debug)]
pub struct Function {
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub return_type: Expression,
    pub body: Expression,
}

impl Function {
    pub fn parse(parser: &mut Parser) -> Self {
        parser.expect_token(TokenKind::Fn);
        let name = Identifier::parse(parser);

        let parameters = Parameter::parse_vec(parser);

        let return_type = Expression::parse_type_specifier(parser);

        parser.expect_token(TokenKind::OpenBrace);
        let body = Expression::parse(parser);
        parser.expect_token(TokenKind::CloseBrace);

        Self {
            name,
            parameters,
            return_type,
            body,
        }
    }
}
