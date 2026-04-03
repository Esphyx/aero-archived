use std::collections::HashMap;

use ast::{
    expression::Expression, function::Function, identifier::Identifier, namespace::Namespace,
};

use crate::lowering::expr::{Builtin, ConsRef, Ref, Expr};

pub struct DeBruijnContext {
    pub local: LocalContext,
    pub global: GlobalContext,
}

impl DeBruijnContext {
    pub fn new(namespace: &Namespace) -> Self {
        Self {
            local: LocalContext::new(),
            global: GlobalContext::new(namespace),
        }
    }

    pub fn convert_term(&mut self, source: &Expression) -> Expr {
        match source {
            Expression::Lambda {
                parameter,
                type_specifier,
                body,
            } => {
                let typ = Box::new(Some(self.convert_term(type_specifier)));
                self.local.push(parameter.clone());

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
                if let Some(index) = self.local.lookup_index(id) {
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

                self.local.push(name.clone());

                let body = self.convert_term(body);

                self.local.pop();

                Expr::construct_application(
                    Expr::construct_binding(typ, body),
                    self.convert_term(value),
                )
            }
            Expression::Builtin(builtin_source) => {
                Expr::Builtin(Builtin::from_source(builtin_source, self))
            }
            Expression::Arrow {
                dependent,
                from_type,
                to_type,
            } => {
                let new_from_type = self.convert_term(from_type);

                if let Some(name) = dependent {
                    self.local.push(name.clone());
                }
                let new_to_type = self.convert_term(to_type);

                if let Some(_) = dependent {
                    self.local.pop();
                }

                Expr::Pi {
                    from_type: Box::new(new_from_type),
                    to_type: Box::new(new_to_type),
                }
            }
            Expression::App { function, argument } => Expr::App {
                func: Box::new(self.convert_term(function)),
                arg: Box::new(self.convert_term(argument)),
            },
        }
    }
}

pub struct LocalContext {
    stack: Vec<Identifier>,
}

impl LocalContext {
    // TODO: store the index/size from the time of insertion
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, id: Identifier) {
        self.stack.push(id);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn lookup_index(&self, id: &Identifier) -> Option<usize> {
        // Later for better errors: while matching names down the stack, save information on closely matching names
        // or for efficiency only when it fails at symbol resolution
        self.stack
            .iter()
            .rev()
            .position(|bound| bound.get_name_str() == id.get_name_str())
    }
}

pub struct GlobalContext {
    constants: HashMap<String, usize>,
    inductives: HashMap<String, usize>,
    constructors: HashMap<String, (usize, usize)>,
}

impl GlobalContext {
    pub fn new(namespace: &Namespace) -> Self {
        let constants = namespace
            .functions
            .iter()
            .enumerate()
            .map(|(i, Function { name, .. })| (name.get_name_str().into(), i))
            .collect();

        let inductives = namespace
            .inductives
            .iter()
            .enumerate()
            .map(|(i, ind)| (ind.name.get_name_str().into(), i))
            .collect();

        let constructors = namespace
            .inductives
            .iter()
            .enumerate()
            .flat_map(|(inductive_index, source_inductive)| {
                source_inductive.constructors.iter().enumerate().map(
                    move |(constructor_index, con)| {
                        (
                            con.name.get_name_str().into(),
                            (inductive_index, constructor_index),
                        )
                    },
                )
            })
            .collect();

        Self {
            constants,
            inductives,
            constructors,
        }
    }

    pub fn resolve(&self, id: &Identifier) -> Option<Ref> {
        let name = id.get_name_str();

        if let Some(&index) = self.constants.get(name) {
            Some(Ref::Fn(index))
        } else if let Some(&index) = self.inductives.get(name) {
            Some(Ref::Ind(index))
        } else if let Some(&(inductive, constructor)) = self.constructors.get(name) {
            Some(Ref::Cons(ConsRef {
                inductive,
                constructor,
            }))
        } else {
            None
        }
    }
}
