use ast::{
    ast::AST, function::Function as SourceFunction, inductive::Inductive as SourceInductive,
    namespace::Namespace as SourceNamespace,
};

use crate::lowering::{
    Lower, de_bruijn::DeBruijnContext, expr::Expr, function::Function, inductive::Inductive,
};

#[derive(Debug)]
pub struct Program {
    pub namespace: Namespace,
}

impl Program {
    pub fn from(ast: &AST) -> Self {
        let mut context = DeBruijnContext::new(&ast.namespace);

        let namespace = Namespace::from(&ast.namespace, &mut context);

        Self { namespace }
    }
}

#[derive(Debug)]
pub struct Namespace {
    pub inductives: Vec<Inductive>,
    pub functions: Vec<Function>,
}

impl Namespace {
    pub fn from(source: &SourceNamespace, context: &mut DeBruijnContext) -> Self {
        let inductives = Self::convert_inductives(&source.inductives, context);
        let functions = Self::convert_functions(&source.functions, context);

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
