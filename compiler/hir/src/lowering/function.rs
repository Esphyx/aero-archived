use ast::{
    expression::Binder, function::Function as SourceFunction, identifier::Identifier,
    parameter::Parameter,
};

use crate::lowering::{Lower, de_bruijn::Context, expr::Expr};

#[derive(Debug)]
pub struct Function {
    pub name: Identifier,
    pub definition: Expr,
    pub return_type: Expr,
}

impl Lower<Function> for SourceFunction {
    fn lower(&self, ctx: &mut Context) -> Function {
        let mut parameter_types = Vec::new();
        for Parameter { name, typ } in self.parameters.iter() {
            parameter_types.push(ctx.convert_term(typ));
            ctx.local.push(Binder::Named(name.clone()));
        }

        let mut return_type = ctx.convert_term(&self.return_type);
        for typ in parameter_types.iter().rev() {
            return_type = Expr::construct_pi(typ.clone(), return_type);
        }

        let mut definition = ctx.convert_term(&self.body);

        for _ in 0..self.parameters.len() {
            ctx.local.pop();
        }

        for typ in parameter_types.iter().rev() {
            definition = Expr::construct_binding(Some(typ.clone()), definition);
        }

        Function {
            name: self.name.clone(),
            return_type,
            definition,
        }
    }
}
