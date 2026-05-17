use std::collections::HashMap;

use syntax::{function::Function, identifier::Identifier, namespace::Namespace};

use crate::lowering::expr::{ConsRef, Ref};

pub struct GlobalContext {
    functions: HashMap<String, usize>,
    inductives: HashMap<String, usize>,
    constructors: HashMap<String, (usize, usize)>,
}

impl GlobalContext {
    pub fn from(namespace: &Namespace) -> Self {
        let functions = namespace
            .functions
            .iter()
            .enumerate()
            .map(|(i, Function { name, .. })| (name.to_string(), i))
            .collect();

        let inductives = namespace
            .inductives
            .iter()
            .enumerate()
            .map(|(i, ind)| (ind.name.to_string(), i))
            .collect();

        let constructors = namespace
            .inductives
            .iter()
            .enumerate()
            .flat_map(|(inductive_index, source_inductive)| {
                source_inductive.constructors.iter().enumerate().map(
                    move |(constructor_index, con)| {
                        (con.name.to_string(), (inductive_index, constructor_index))
                    },
                )
            })
            .collect();

        Self {
            functions,
            inductives,
            constructors,
        }
    }

    pub fn resolve(&self, id: &Identifier) -> Option<Ref> {
        let name = id.to_string();

        if let Some(&index) = self.functions.get(&name) {
            Some(Ref::Fn(index))
        } else if let Some(&index) = self.inductives.get(&name) {
            Some(Ref::Ind(index))
        } else if let Some(&(inductive, constructor)) = self.constructors.get(&name) {
            Some(Ref::Cons(ConsRef {
                ind: inductive,
                cons: constructor,
            }))
        } else {
            None
        }
    }
}
