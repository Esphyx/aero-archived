use lexer::token::TokenKind;
use parser::{error::ParseError, parser::Parser};

use crate::{function::Function, inductive::Inductive};

#[derive(Debug)]
pub struct Namespace {
    pub inductives: Vec<Inductive>,
    pub functions: Vec<Function>,
}

impl Namespace {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut inductives = Vec::new();
        let mut functions = Vec::new();

        while !matches!(parser.current().kind, TokenKind::EoF) {
            let current_token = parser.current();
            match current_token.kind {
                TokenKind::Inductive => {
                    inductives.push(Inductive::parse(parser)?);
                }
                TokenKind::Fn => {
                    functions.push(Function::parse(parser)?);
                }
                TokenKind::Comment => {
                    parser.advance();
                }
                _ => {
                    panic!("Expected token!");
                }
            }
        }

        Ok(Self {
            inductives,
            functions,
        })
    }
}
