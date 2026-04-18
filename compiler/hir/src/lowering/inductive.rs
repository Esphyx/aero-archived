use ast::{
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
        for p in &self.parameters {
            ctx.local.push(p.name.clone());
        }

        let constructors = self.constructors.iter().map(|c| c.lower(ctx)).collect();

        let mut typ = ctx.convert_term(&self.typ);

        for p in self.parameters.iter().rev() {
            let param_ty = ctx.convert_term(&p.typ);
            typ = Expr::Pi {
                from_type: Box::new(param_ty),
                to_type: Box::new(typ),
            }
        }

        for _ in &self.parameters {
            ctx.local.pop();
        }

        Inductive {
            name: self.name.clone(),
            typ,
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
