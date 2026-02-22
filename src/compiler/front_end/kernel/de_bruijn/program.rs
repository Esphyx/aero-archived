use crate::compiler::front_end::{
    grammar::ast::{
        SourceAST, function::SourceFunction, inductive::SourceInductive, namespace::SourceNamespace,
    },
    kernel::de_bruijn::{constant::Function, context::DeBruijnContext, inductive::Inductive},
};

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
        let (entry_point, functions) =
            Self::convert_functions(&source.constants, context, entry_point_name);
        let inductives = Self::convert_inductives(&source.inductives, context);
        (
            entry_point,
            Self {
                inductives,
                functions,
            },
        )
    }

    pub fn convert_functions(
        source: &Vec<SourceFunction>,
        context: &mut DeBruijnContext,
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

            functions.push(Function::from(func, context));
        }
        (entry_point, functions)
    }

    pub fn convert_inductives(
        source: &Vec<SourceInductive>,
        context: &mut DeBruijnContext,
    ) -> Vec<Inductive> {
        source.iter().map(|s| Inductive::from(s, context)).collect()
    }
}
