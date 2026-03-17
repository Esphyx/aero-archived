use ast::{function::SourceFunction, identifier::SourceIdentifier, parameter::Parameter};

use crate::lowering::{Lower, de_bruijn::DeBruijnContext, term::Term};


#[derive(Debug)]
pub struct Function {
    pub name: SourceIdentifier,
    pub parameter_types: Vec<Term>,
    pub return_type: Option<Term>,
    pub definition: Term,
}

impl Function {
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

impl Lower<Function> for SourceFunction {
    fn lower(&self, ctx: &mut DeBruijnContext) -> Function {
        let mut parameter_types = Vec::new();
        for Parameter { name, typ } in self.parameters.iter() {
            parameter_types.push(ctx.convert_term(typ));
            ctx.local.push(name.clone());
        }

        let return_type = self.return_type.as_ref().map(|s| ctx.convert_term(&s));

        let definition = ctx.convert_term(&self.body);

        for _ in 0..self.parameters.len() {
            ctx.local.pop();
        }

        Function {
            name: self.name.clone(),
            parameter_types,
            return_type,
            definition,
        }
    }
}
