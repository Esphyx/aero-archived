use ast::{
    identifier::Identifier,
    inductive::{Inductive as SourceInductive, Constructor as SourceConstructor},
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
        self.parameters.iter().for_each(|p| {
            ctx.local.push(p.name.clone());
        });

        let constructors = self.constructors.iter().map(|s| s.lower(ctx)).collect();

        self.parameters.iter().for_each(|_| {
            ctx.local.pop();
        });

        Inductive {
            name: self.name.clone(),
            typ: ctx.convert_term(&self.typ),
            constructors,
        }
    }
}
impl Lower<Constructor> for SourceConstructor {
    fn lower(&self, ctx: &mut DeBruijnContext) -> Constructor {
        Constructor {
            name: self.name.clone(),
            typ: ctx.convert_term(&self.typ),
        }
    }
}
