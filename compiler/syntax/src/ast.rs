use std::fmt::Debug;

use parser::{error::ParseError, parser::Parser};

use crate::namespace::Namespace;

#[derive(Debug)]
pub struct Syntax {
    pub namespace: Namespace,
}

impl Syntax {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        Ok(Self {
            namespace: Namespace::parse(parser)?,
        })
    }
}
