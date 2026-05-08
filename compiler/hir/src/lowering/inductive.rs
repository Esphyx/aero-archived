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
        let mut binder_ids = Vec::new();

        for p in self.parameters.iter() {
            let binder_id = ctx.local.extend(Binder::Named(p.name.clone()));
            binder_ids.push(binder_id);
        }

        let mut typ = ctx.convert_term(&self.typ);

        for (p, binder_id) in self.parameters.iter().zip(binder_ids.iter()).rev() {
            ctx.local.pop();
            typ = Expr::construct_pi(Some(*binder_id), ctx.convert_term(&p.typ), typ);
        }

        let mut binder_ids = Vec::new();
        for p in self.parameters.iter() {
            let binder_id = ctx.local.extend(Binder::Named(p.name.clone()));
            binder_ids.push(binder_id);
        }

        let mut constructors_types: Vec<Expr> = self
            .constructors
            .iter()
            .map(|c| ctx.convert_term(&c.typ))
            .collect();

        for (p, binder_id) in self.parameters.iter().zip(binder_ids.iter()).rev() {
            ctx.local.pop();

            let param_ty = ctx.convert_term(&p.typ);

            constructors_types = constructors_types
                .into_iter()
                .map(|t| Expr::construct_pi(Some(*binder_id), param_ty.clone(), t))
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
