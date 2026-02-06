use super::{super::super::grammar::ast::identifier::SourceIdentifier, term::Term};

#[derive(Debug)]
pub struct Inductive {
    pub name: SourceIdentifier,
    pub typ: Term,
    pub constructors: Vec<Constructor>,
    pub eliminator: SourceIdentifier,
}

#[derive(Debug)]
pub struct Constructor {
    pub name: SourceIdentifier,
    pub typ: Term,
}
