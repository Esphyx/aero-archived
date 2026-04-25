use ast::{
    ast::AST, function::Function as SourceFunction, inductive::Inductive as SourceInductive,
};

use crate::lowering::{
    Lower, de_bruijn::DeBruijnContext, function::Function, inductive::Inductive,
};

#[derive(Debug)]
pub struct Namespace {
    pub inductives: Vec<Inductive>,
    pub functions: Vec<Function>,
}

impl Namespace {
    pub fn from(ast: &AST) -> Self {
        let mut context = DeBruijnContext::new(&ast.namespace);

        let inductives = Self::convert_inductives(&ast.namespace.inductives, &mut context);
        let functions = Self::convert_functions(&ast.namespace.functions, &mut context);

        Self {
            functions,
            inductives,
        }
    }

    pub fn convert_functions(
        source: &Vec<SourceFunction>,
        ctx: &mut DeBruijnContext,
    ) -> Vec<Function> {
        source.iter().map(|s| s.lower(ctx)).collect()
    }

    pub fn convert_inductives(
        source: &Vec<SourceInductive>,
        ctx: &mut DeBruijnContext,
    ) -> Vec<Inductive> {
        source.iter().map(|s| s.lower(ctx)).collect()
    }
}
