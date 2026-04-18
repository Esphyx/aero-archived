use std::fmt::Debug;

use parser::{error::ParseError, parser::Parser};

use crate::namespace::Namespace;

#[derive(Debug)]
pub struct AST {
    pub namespace: Namespace,
}

impl AST {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        Ok(Self {
            namespace: Namespace::parse(parser)?,
        })
    }
}
