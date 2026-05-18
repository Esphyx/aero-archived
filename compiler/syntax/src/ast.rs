use std::fmt::Debug;

use parser::parser::Parser;

use crate::namespace::Namespace;

#[derive(Debug)]
pub struct Syntax {
    pub namespace: Namespace,
}

impl Syntax {
    pub fn parse(parser: &mut Parser) -> Self {
        Self {
            namespace: Namespace::parse(parser),
        }
    }
}
