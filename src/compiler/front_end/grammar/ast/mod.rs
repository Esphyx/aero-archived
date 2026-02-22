use std::fmt::Debug;

use {
    super::{
        parser::{ParseError, Parser},
        token::TokenKind,
    },
    builtin::{SourceBuiltinType, SourcePrimitive, SourceBuiltin},
    identifier::SourceIdentifier,
    namespace::SourceNamespace,
    parameter::Parameter,
    term::SourceTerm,
};

pub mod builtin;
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
