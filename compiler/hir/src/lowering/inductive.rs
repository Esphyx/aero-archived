use ast::{
    expression::Binder,
    identifier::Identifier,
    inductive::{Constructor as SourceConstructor, Inductive as SourceInductive},
};

use crate::lowering::{Lower, de_bruijn::DeBruijnContext, expr::Expr};

#[derive(Debug)]
pub struct Inductive {
    pub name: Identifier,
    pub typ: Expr,
    pub constructors: Vec<Constructor>,
}

#[derive(Debug)]
pub struct Constructor {
    pub name: Identifier,
    pub typ: Expr,
}

impl Lower<Inductive> for SourceInductive {
    fn lower(&self, ctx: &mut DeBruijnContext) -> Inductive {
        for p in self.parameters.iter() {
            ctx.local.push(Binder::Named(p.name.clone()));
        }

        let mut typ = ctx.convert_term(&self.typ);

        for p in self.parameters.iter().rev() {
            ctx.local.pop();
            typ = Expr::construct_pi(ctx.convert_term(&p.typ), typ)
        }

        Inductive {
            name: self.name.clone(),
            typ,
            constructors: Vec::new(),
        }
    }
}
