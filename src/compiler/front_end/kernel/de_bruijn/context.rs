use std::collections::HashMap;

use crate::compiler::front_end::{
    grammar::ast::term::SourceTerm,
    kernel::de_bruijn::{builtin::Builtin, term::Term},
};

use super::{
    super::super::grammar::ast::{
        function::SourceFunction, identifier::SourceIdentifier, namespace::SourceNamespace,
    },
    global_ref::GlobalRef,
};

pub struct DeBruijnContext {
    pub local: LocalContext,
    global: GlobalContext,
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
                    panic!("Unbound identifier '{}'!", id.get_name_str());
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
}

impl GlobalContext {
    pub fn new(namespace: &SourceNamespace) -> Self {
        let constants = namespace
            .constants
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

        Self {
            constants,
            inductives,
        }
    }

    pub fn resolve(&self, id: &SourceIdentifier) -> Option<GlobalRef> {
        let name = id.get_name_str();

        if let Some(&index) = self.constants.get(name) {
            Some(GlobalRef::ConstRef(index))
        } else if let Some(&index) = self.inductives.get(name) {
            Some(GlobalRef::InductiveRef(index))
        } else {
            None
        }
    }
}
