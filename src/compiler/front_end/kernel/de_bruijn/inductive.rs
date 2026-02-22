use crate::compiler::front_end::{
    grammar::ast::inductive::{SourceConstructor, SourceInductive},
    kernel::de_bruijn::context::DeBruijnContext,
};

use super::{super::super::grammar::ast::identifier::SourceIdentifier, term::Term};

#[derive(Debug)]
pub struct Inductive {
    pub name: SourceIdentifier,
    pub typ: Term,
    pub constructors: Vec<Constructor>,
    pub eliminator: SourceIdentifier,
}

impl Inductive {
    pub fn from(source: &SourceInductive, context: &mut DeBruijnContext) -> Self {
        Self {
            name: source.name.clone(),
            typ: context.convert_term(&source.typ),
            constructors: source
                .constructors
                .iter()
                .map(|s| Constructor::from(s, context))
                .collect(),
            eliminator: source.eliminator.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Constructor {
    pub name: SourceIdentifier,
    pub typ: Term,
}

impl Constructor {
    pub fn from(source: &SourceConstructor, context: &mut DeBruijnContext) -> Self {
        Self {
            name: source.name.clone(),
            typ: context.convert_term(&source.typ),
        }
    }
}
