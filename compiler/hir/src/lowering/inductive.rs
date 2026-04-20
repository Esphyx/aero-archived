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
        for p in self.parameters.iter() {
            ctx.local.push(p.name.clone());
        }

        let mut typ = ctx.convert_term(&self.typ);

        // issue is likely pi conversion
        // todo: constructor conversion

        Inductive {
            name: self.name.clone(),
            typ,
            constructors: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use ast::{identifier::Identifier, inductive::Inductive as SourceInductive};

    #[test]
    fn convert() {
        
    }
}
