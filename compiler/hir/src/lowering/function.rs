use ast::{expression::Binder, function::Function as SourceFunction, identifier::Identifier, parameter::Parameter};

use crate::lowering::{Lower, de_bruijn::DeBruijnContext, expr::Expr};

#[derive(Debug)]
pub struct Function {
    pub name: Identifier,
    pub parameter_types: Vec<Expr>,
    pub return_type: Option<Expr>,
    pub definition: Expr,
}

impl Function {
    pub fn wrap_with_lambdas(&self) -> Expr {
        let mut body = self.definition.clone();
        for typ in self.parameter_types.iter() {
            body = Expr::Lambda {
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
            ctx.local.push(Binder::Named(name.clone()));
        }

        let return_type = Some(ctx.convert_term(&self.return_type));

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
