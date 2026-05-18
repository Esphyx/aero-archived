use lexer::token::TokenKind;
use parser::parser::Parser;

use crate::{function::Function, inductive::Inductive};

#[derive(Debug)]
pub struct Namespace {
    pub inductives: Vec<Inductive>,
    pub functions: Vec<Function>,
}

impl Namespace {
    pub fn parse(parser: &mut Parser) -> Self {
        let mut inductives = Vec::new();
        let mut functions = Vec::new();

        while !matches!(parser.peek_kind(), TokenKind::EoF) {
            match parser.peek_kind() {
                TokenKind::Inductive => {
                    inductives.push(Inductive::parse(parser));
                }
                TokenKind::Fn => {
                    functions.push(Function::parse(parser));
                }
                TokenKind::Comment => {
                    parser.advance();
                }
                _ => {
                    parser.error("expected token");
                    parser.advance();
                }
            }
        }

        Self {
            inductives,
            functions,
        }
    }
}
