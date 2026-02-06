use std::fmt::Debug;

use {
    super::{
        parser::{ParseError, Parser},
        token::TokenKind,
    },
    identifier::SourceIdentifier,
    namespace::SourceNamespace,
    term::SourceTerm,
    parameter::Parameter
};

pub mod function;
pub mod identifier;
pub mod inductive;
pub mod namespace;
pub mod parameter;
pub mod term;

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

#[derive(Debug, Clone)]
pub enum SourceBuiltinType {
    Prop,
    Type(u32),
    U8,
    Unit,
    Bool,
    Array { dependent: Box<SourceTerm> },
}
