use std::fmt::Debug;

use parser::parser::{ParseError, Parser};

use crate::namespace::SourceNamespace;

#[derive(Debug)]
pub struct SourceAST {
    pub namespace: SourceNamespace,
}

impl SourceAST {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        Ok(Self {
            namespace: SourceNamespace::parse(parser)?,
        })
    }
}
