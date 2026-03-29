use lexer::token::TokenKind;
use parser::parser::{ParseError, Parser};

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
                    // return Err(ParseError::ExpectedToken {
                    //     expected: vec![TokenKind::Fn, TokenKind::Inductive],
                    //     found: current_token.clone(),
                    //     position: current_token.position,
                    // });
                }
            }
        }

        Ok(Self {
            inductives,
            functions,
        })
    }
}
