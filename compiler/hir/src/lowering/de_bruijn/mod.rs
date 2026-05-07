use ast::{
    expression::{Binder, Builtin as SourceBuiltin, Expression},
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
            global: GlobalContext::new(namespace),
        }
    }

    pub fn convert_term(&mut self, source: &Expression) -> Expr {
        match source {
            Expression::Lambda {
                param: binder,
                type_specifier,
                body,
            } => {
                let typ = Box::new(Some(self.convert_term(type_specifier)));
                self.local.extend(binder.clone());
                let body = Box::new(self.convert_term(body));
                self.local.pop();
                Expr::Lambda { typ, body }
            }
            Expression::Match {
                scrutinee,
                branches,
            } => {
                let scrutinee = Box::new(self.convert_term(scrutinee));

                let branches: Vec<(ConsRef, Expr)> = branches
                    .iter()
                    .map(|branch| {
                        let cons_ref = match self
                            .global
                            .resolve(&branch.pattern)
                            .expect("Unknown constructor in match!")
                        {
                            Ref::Cons(r) => r,
                            _ => panic!("Pattern must be a constructor!"),
                        };
                        let body_term = self.convert_term(&branch.body);

                        (cons_ref, body_term)
                    })
                    .collect();

                Expr::Match {
                    scrutinee,
                    branches,
                }
            }
            Expression::Identifier(id) => {
                if let Some(index) = self.local.resolve(id) {
                    Expr::Var(index)
                } else if let Some(global_ref) = self.global.resolve(id) {
                    Expr::Ref(global_ref)
                } else {
                    panic!("Unbound identifier '{}'! {:?}", id.get_name_str(), source);
                }
            }
            Expression::Let {
                name,
                value,
                type_specifier,
                body,
            } => {
                let typ = type_specifier
                    .as_ref()
                    .as_ref()
                    .map(|t| self.convert_term(&t));

                self.local.extend(Binder::Named(name.clone()));

                let body = self.convert_term(body);

                self.local.pop();

                Expr::construct_application(
                    Expr::construct_binding(typ, body),
                    self.convert_term(value),
                )
            }
            Expression::Builtin(builtin) => Expr::Builtin(match builtin {
                SourceBuiltin::Prop => Builtin::Prop,
                SourceBuiltin::Type(u) => Builtin::Type(*u),
            }),
            Expression::Arrow {
                dependent,
                typ,
                body,
            } => {
                let new_typ = self.convert_term(typ);
                self.local.extend(dependent.clone());
                let new_body = self.convert_term(body);
                self.local.pop();
                Expr::construct_pi(new_typ, new_body)
            }
            Expression::App { func, arg } => {
                Expr::construct_application(self.convert_term(func), self.convert_term(arg))
            }
        }
    }
}
