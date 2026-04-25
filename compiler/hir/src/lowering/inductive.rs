use ast::{expression::Binder, identifier::Identifier, inductive::Inductive as SourceInductive};

use crate::lowering::{Lower, de_bruijn::Context, expr::Expr};

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
    fn lower(&self, ctx: &mut Context) -> Inductive {
        for p in self.parameters.iter() {
            ctx.local.push(Binder::Named(p.name.clone()));
        }

        let mut typ = ctx.convert_term(&self.typ);

        for p in self.parameters.iter().rev() {
            ctx.local.pop();
            typ = Expr::construct_pi(ctx.convert_term(&p.typ), typ);
        }

        for p in self.parameters.iter() {
            ctx.local.push(Binder::Named(p.name.clone()));
        }

        let mut constructors_types: Vec<Expr> = self
            .constructors
            .iter()
            .map(|c| ctx.convert_term(&c.typ))
            .collect();

        for p in self.parameters.iter().rev() {
            ctx.local.pop();

            constructors_types = constructors_types
                .into_iter()
                .map(|t| Expr::construct_pi(ctx.convert_term(&p.typ), t))
                .collect();
        }

        let constructors = constructors_types
            .into_iter()
            .zip(&self.constructors)
            .map(|(t, c)| Constructor {
                name: c.name.clone(),
                typ: t,
            })
            .collect();

        Inductive {
            name: self.name.clone(),
            typ,
            constructors,
        }
    }
}
