use ast::{
    expression::{Binder, Branch, Builtin as SourceBuiltin, Expression},
    namespace::Namespace,
};

use crate::lowering::{
    de_bruijn::{global::GlobalContext, local::LocalContext},
    expr::{Builtin, ConsRef, Expr, Ref},
};

pub mod global;
pub mod local;

pub struct Context {
    pub local: LocalContext,
    pub global: GlobalContext,
}

impl Context {
    pub fn new(namespace: &Namespace) -> Self {
        Self {
            local: LocalContext::new(),
            global: GlobalContext::from(namespace),
        }
    }

    pub fn convert_term(&mut self, source: &Expression) -> Expr {
        match source {
            Expression::Identifier(id) => {
                if let Some((index, binder)) = self.local.resolve(id) {
                    Expr::Var {
                        index,
                        binder: Some(binder),
                    }
                } else if let Some(global_ref) = self.global.resolve(id) {
                    Expr::Ref(global_ref)
                } else {
                    panic!("Unbound identifier '{}'! {:?}", id.get_name_str(), source);
                }
            }
            Expression::Lambda {
                param,
                type_specifier,
                body,
            } => self.convert_lambda(param, type_specifier, body),
            Expression::Pi {
                dependent,
                typ,
                body,
            } => self.convert_pi(dependent, typ, body),
            Expression::Match {
                scrutinee,
                branches,
            } => self.convert_match(scrutinee, branches),
            Expression::Builtin(builtin) => Expr::Builtin(match builtin {
                SourceBuiltin::Prop => Builtin::Prop,
                SourceBuiltin::Type(u) => Builtin::Type(*u),
            }),
            Expression::App { func, arg } => {
                Expr::construct_application(self.convert_term(func), self.convert_term(arg))
            }
        }
    }

    fn convert_lambda(
        &mut self,
        binder: &Binder,
        type_specifier: &Expression,
        body: &Expression,
    ) -> Expr {
        let typ = self.convert_term(type_specifier);
        let id = self.local.extend(binder.clone());
        let body = self.convert_term(body);
        self.local.pop();
        Expr::construct_lambda(Some(id), typ, body)
    }

    fn convert_pi(&mut self, dependent: &Binder, typ: &Expression, body: &Expression) -> Expr {
        let new_typ = self.convert_term(typ);
        let id = self.local.extend(dependent.clone());
        let new_body = self.convert_term(body);
        self.local.pop();
        Expr::construct_pi(Some(id), new_typ, new_body)
    }

    fn convert_match(&mut self, scrutinee: &Expression, branches: &Vec<Branch>) -> Expr {
        let scrutinee = Box::new(self.convert_term(scrutinee));

        let branches: Vec<(ConsRef, Expr)> = branches
            .iter()
            .map(|branch| {
                if let Ref::Cons(r) = self
                    .global
                    .resolve(&branch.pattern)
                    .expect("Unknown constructor!")
                {
                    (r, self.convert_term(&branch.body))
                } else {
                    panic!("Pattern must be a constructor!")
                }
            })
            .collect();

        Expr::Match {
            scrutinee,
            branches,
        }
    }
}
