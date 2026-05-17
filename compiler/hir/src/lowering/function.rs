use syntax::{
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
        let mut binder_ids = Vec::new();
        for Parameter { name, typ } in self.parameters.iter() {
            parameter_types.push(ctx.convert_term(typ));
            let binder_id = ctx.local.extend(Binder::Named(name.clone()));
            binder_ids.push(binder_id);
        }

        let mut return_type = ctx.convert_term(&self.return_type);
        let mut definition = ctx.convert_term(&self.body);

        for _ in 0..self.parameters.len() {
            ctx.local.pop();
        }

        for (typ, binder_id) in parameter_types.iter().zip(binder_ids.iter()).rev() {
            return_type = Expr::construct_pi(Some(*binder_id), typ.clone(), return_type);
            definition = Expr::construct_lambda(Some(*binder_id), typ.clone(), definition);
        }

        Function {
            name: self.name.clone(),
            return_type,
            definition,
        }
    }
}
