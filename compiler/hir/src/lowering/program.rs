use ast::{ast::SourceAST, function::SourceFunction, inductive::SourceInductive, namespace::SourceNamespace};

use crate::lowering::{Lower, de_bruijn::DeBruijnContext, function::Function, inductive::Inductive};


#[derive(Debug)]
pub struct Program {
    pub namespace: Namespace,
    pub entry_point: usize,
}

impl Program {
    pub fn from(ast: &SourceAST) -> Self {
        let mut context = DeBruijnContext::new(&ast.namespace);

        const ENTRY_POINT_NAME: &str = "main";
        let (entry_point, namespace) =
            Namespace::from(&ast.namespace, &mut context, ENTRY_POINT_NAME);

        Self {
            namespace,
            entry_point: entry_point.expect("Entry point not found!"),
        }
    }
}

#[derive(Debug)]
pub struct Namespace {
    pub inductives: Vec<Inductive>,
    pub functions: Vec<Function>,
}

impl Namespace {
    pub fn from(
        source: &SourceNamespace,
        context: &mut DeBruijnContext,
        entry_point_name: &str,
    ) -> (Option<usize>, Self) {
        let inductives = Self::convert_inductives(&source.inductives, context);
        let (entry_point, functions) =
            Self::convert_functions(&source.functions, context, entry_point_name);
        (
            entry_point,
            Self {
                functions,
                inductives,
            },
        )
    }

    pub fn convert_functions(
        source: &Vec<SourceFunction>,
        ctx: &mut DeBruijnContext,
        entry_point_name: &str,
    ) -> (Option<usize>, Vec<Function>) {
        let mut entry_point = None;
        let mut functions = Vec::new();
        for (i, func) in source.iter().enumerate() {
            if func.name.get_name_str() == entry_point_name {
                if !func.parameters.is_empty() {
                    panic!("Main function mustn't have arguments!");
                }
                entry_point = Some(i);
            }

            ctx.global.current_function = Some(i);
            functions.push(func.lower(ctx));
            ctx.global.current_function = None;
        }
        (entry_point, functions)
    }

    pub fn convert_inductives(
        source: &Vec<SourceInductive>,
        ctx: &mut DeBruijnContext,
    ) -> Vec<Inductive> {
        source
            .iter()
            .enumerate()
            .map(|(i, s)| {
                ctx.global.current_inductive = Some(i);
                let ind = s.lower(ctx);
                ctx.global.current_inductive = None;
                ind
            })
            .collect()
    }
}
