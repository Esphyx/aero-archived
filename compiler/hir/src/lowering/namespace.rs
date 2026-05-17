use syntax::{
    ast::Syntax, function::Function as SourceFunction, inductive::Inductive as SourceInductive,
};

use crate::lowering::{Lower, de_bruijn::Context, function::Function, inductive::Inductive};

#[derive(Debug)]
pub struct Namespace {
    pub inductives: Vec<Inductive>,
    pub functions: Vec<Function>,
}

impl Namespace {
    pub fn from(ast: &Syntax) -> (Self, Context) {
        let mut context = Context::new(&ast.namespace);

        let inductives = Self::convert_inductives(&ast.namespace.inductives, &mut context);
        let functions = Self::convert_functions(&ast.namespace.functions, &mut context);

        (
            Self {
                functions,
                inductives,
            },
            context,
        )
    }

    pub fn convert_functions(source: &Vec<SourceFunction>, ctx: &mut Context) -> Vec<Function> {
        source.iter().map(|s| s.lower(ctx)).collect()
    }

    pub fn convert_inductives(source: &Vec<SourceInductive>, ctx: &mut Context) -> Vec<Inductive> {
        source.iter().map(|s| s.lower(ctx)).collect()
    }
}
