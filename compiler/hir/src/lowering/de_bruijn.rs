use std::collections::HashMap;

use ast::{
    function::SourceFunction, identifier::SourceIdentifier, namespace::SourceNamespace,
    term::SourceTerm,
};

use crate::lowering::term::{Builtin, GlobalRef, Term};

pub struct DeBruijnContext {
    pub local: LocalContext,
    pub global: GlobalContext,
}

impl DeBruijnContext {
    pub fn new(namespace: &SourceNamespace) -> Self {
        Self {
            local: LocalContext::new(),
            global: GlobalContext::new(namespace),
        }
    }

    pub fn convert_term(&mut self, source: &SourceTerm) -> Term {
        match source {
            SourceTerm::Identifier(id) => {
                if let Some(index) = self.local.lookup_index(id) {
                    Term::Var(index)
                } else if let Some(global_ref) = self.global.resolve(id) {
                    Term::GlobalRef(global_ref)
                } else {
                    panic!("Unbound identifier '{}'! {:?}", id.get_name_str(), source);
                }
            }

            SourceTerm::Let {
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

                Term::construct_application(
                    Term::construct_binding(typ, body),
                    self.convert_term(value),
                )
            }

            SourceTerm::Builtin(builtin_source) => {
                Term::Builtin(Builtin::from_source(builtin_source, self))
            }

            SourceTerm::Arrow {
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

                Term::Pi {
                    from_type: Box::new(new_from_type),
                    to_type: Box::new(new_to_type),
                }
            }

            SourceTerm::App { function, argument } => Term::App {
                function: Box::new(self.convert_term(function)),
                argument: Box::new(self.convert_term(argument)),
            },
        }
    }
}

pub struct LocalContext {
    stack: Vec<SourceIdentifier>,
}

impl LocalContext {
    // TODO: store the index/size from the time of insertion
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, id: SourceIdentifier) {
        self.stack.push(id);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn lookup_index(&self, id: &SourceIdentifier) -> Option<usize> {
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

    pub current_function: Option<usize>,
    pub current_inductive: Option<usize>,
}

impl GlobalContext {
    pub fn new(namespace: &SourceNamespace) -> Self {
        let constants = namespace
            .functions
            .iter()
            .enumerate()
            .map(|(i, SourceFunction { name, .. })| (name.get_name_str().into(), i))
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
            current_function: None,
            current_inductive: None,
        }
    }

    pub fn resolve(&self, id: &SourceIdentifier) -> Option<GlobalRef> {
        let name = id.get_name_str();

        if let Some(&index) = self.constants.get(name) {
            if let Some(f) = self.current_function {
                if f == index {
                    return Some(GlobalRef::SelfRef(f));
                }
            }
            Some(GlobalRef::ConstRef(index))
        } else if let Some(&index) = self.inductives.get(name) {
            if let Some(ind) = self.current_inductive {
                if ind == index {
                    return Some(GlobalRef::SelfRef(ind));
                }
            }
            Some(GlobalRef::InductiveRef(index))
        } else if let Some(&(inductive, constructor)) = self.constructors.get(name) {
            Some(GlobalRef::ConstructorRef {
                inductive,
                constructor,
            })
        } else {
            None
        }
    }
}
