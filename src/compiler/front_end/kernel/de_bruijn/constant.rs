use crate::compiler::front_end::{
    grammar::ast::{function::SourceFunction, parameter::Parameter},
    kernel::de_bruijn::context::DeBruijnContext,
};

use super::{super::super::grammar::ast::identifier::SourceIdentifier, term::Term};

#[derive(Debug)]
pub struct Function {
    pub name: SourceIdentifier,
    pub parameter_types: Vec<Term>,
    pub return_type: Option<Term>,
    pub definition: Term,
}

impl Function {
    pub fn from(source: &SourceFunction, context: &mut DeBruijnContext) -> Self {
        let mut parameter_types = Vec::new();
        for Parameter { name, typ } in source.parameters.iter() {
            parameter_types.push(context.convert_term(typ));
            context.local.push(name.clone());
        }

        let return_type = source
            .return_type
            .as_ref()
            .map(|s| context.convert_term(&s));

        let definition = context.convert_term(&source.body);

        for _ in 0..source.parameters.len() {
            context.local.pop();
        }

        Self {
            name: source.name.clone(),
            parameter_types,
            return_type,
            definition,
        }
    }

    pub fn wrap_with_lambdas(&self) -> Term {
        let mut body = self.definition.clone();
        for typ in self.parameter_types.iter() {
            body = Term::Lambda {
                typ: Box::new(Some(typ.clone())),
                body: Box::new(body),
            }
        }
        body
    }
}
