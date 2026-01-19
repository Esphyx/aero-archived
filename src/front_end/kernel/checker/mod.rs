use std::collections::HashMap;

use crate::front_end::kernel::cic::{Inductive, Term};

#[derive(Clone)]
pub struct Context<'ctx> {
    pub inductives: HashMap<String, &'ctx Inductive>,
    pub constants: HashMap<String, &'ctx Term>,
}

impl<'ctx> Context<'ctx> {
    pub fn new() -> Self {
        Self {
            inductives: HashMap::new(),
            constants: HashMap::new(),
        }
    }

    pub fn insert_constant(&mut self, name: String, term: &'ctx Term) {
        self.constants.insert(name, term);
    }

    pub fn get_constant(&self, name: &str) -> Option<&'ctx Term> {
        self.constants.get(name).map(|t| &**t)
    }

    pub fn remove_constant(&mut self, name: &String) {
        self.constants.remove(name);
    }

    pub fn insert_inductive(&mut self, ind: &'ctx Inductive) {
        self.inductives.insert(ind.name.clone(), ind);
    }
}

pub fn type_of<'ctx>(
    term: &'ctx Term,
    context: &'ctx mut Context<'ctx>,
) -> Result<&'ctx Term, String> {
    match term {
        Term::Identifier(id) => context
            .get_constant(&id.name)
            .ok_or_else(|| format!("Unknown identifier {}", id.name)),

        Term::Const(const_name) => context
            .get_constant(&const_name.0)
            .ok_or_else(|| format!("Unknown constant {}", const_name.0)),

        Term::Universe(u) => Ok(term),

        Term::Pi {
            dependent: param,
            from_type: ty,
            to_type: body,
        } => {
            todo!()
        }

        Term::Lambda {
            parameter,
            r#type,
            body,
        } => {
            todo!()
        }

        Term::App { function, argument } => {
            let mut cloned_context = context.clone();
            let function_type = type_of(function, context)?;

            match function_type {
                Term::Pi {
                    dependent: _,
                    from_type,
                    to_type,
                } => {
                    let argument_type = type_of(argument, &mut cloned_context)?;
                    if &**from_type != argument_type {
                        return Err("Type mismatch in application".into());
                    }
                    Ok(to_type)
                }
                _ => Err("Trying to apply a non-function".into()),
            }
        }
    }
}
