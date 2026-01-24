use std::{collections::HashMap, error::Error, fmt::Display};

use crate::front_end::kernel::parser_ast::{
    SourceAST, SourceBuiltinType, SourceIdentifier, SourceNamespace, SourceTerm,
};

pub struct Context<'a> {
    stack: Vec<SourceIdentifier>,
    input: &'a str,
}

impl<'a> Context<'a> {
    // store index at time of insertion ?
    pub fn new(input: &'a str) -> Self {
        Self {
            stack: Vec::new(),
            input,
        }
    }

    pub fn push(&mut self, id: SourceIdentifier) {
        self.stack.push(id);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn lookup_index(&self, id: &SourceIdentifier) -> Option<usize> {
        // Later for better errors while matching names down the stack save information for closely matching names
        // or for efficiency only when it fails at symbol resolution
        self.stack
            .iter()
            .rev()
            .position(|bound| bound.get_name_str(self.input) == id.get_name_str(self.input))
    }
}

#[derive(Debug)]
pub enum BuiltinTypeDB {
    Prop,
    Type(u32),
    U8,
    Array { dependent: Box<TermDB> },
}

#[derive(Debug)]
pub enum GlobalRef {
    ConstRef(usize),
    InductiveRef(usize),
}

#[derive(Debug)]
pub enum TermDB {
    Var(usize),
    Builtin(BuiltinTypeDB),
    GlobalRef(GlobalRef),
    Pi {
        from_type: Box<TermDB>,
        to_type: Box<TermDB>,
    },
    Lambda {
        r#type: Box<TermDB>,
        body: Box<TermDB>,
    },
    App {
        function: Box<TermDB>,
        argument: Box<TermDB>,
    },
}

#[derive(Debug)]
pub struct ASTDB {
    pub environment: NamespaceDB,
}

#[derive(Debug)]
pub struct NamespaceDB {
    pub inductives: Vec<InductiveDB>,
    pub constants: Vec<(SourceIdentifier, TermDB)>,
}

#[derive(Debug)]
pub struct InductiveDB {
    name: SourceIdentifier,
    r#type: TermDB,
    constructors: Vec<ConstructorDB>,
    eliminator: SourceIdentifier,
}

#[derive(Debug)]
pub struct ConstructorDB {
    name: SourceIdentifier,
    r#type: TermDB,
}

struct GlobalContext<'a> {
    constants: HashMap<&'a str, usize>,
    inductives: HashMap<&'a str, usize>,
    input: &'a str,
}

impl<'a> GlobalContext<'a> {
    fn new(namespace: &SourceNamespace, input: &'a str) -> Self {
        let constants = namespace
            .constants
            .iter()
            .enumerate()
            .map(|(i, (id, _))| (id.get_name_str(input), i))
            .collect();

        let inductives = namespace
            .inductives
            .iter()
            .enumerate()
            .map(|(i, ind)| (ind.name.get_name_str(input), i))
            .collect();

        Self {
            constants,
            inductives,
            input,
        }
    }

    fn resolve(&self, id: &SourceIdentifier) -> Option<GlobalRef> {
        let name = id.get_name_str(self.input);

        if let Some(&index) = self.constants.get(name) {
            Some(GlobalRef::ConstRef(index))
        } else if let Some(&index) = self.inductives.get(name) {
            Some(GlobalRef::InductiveRef(index))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub enum DBError {
    UnboundIdentifier,
}

impl Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for DBError {}

pub fn convert_parser_ast_to_db<'a>(ast: &SourceAST, input: &'a str) -> Result<ASTDB, DBError> {
    let namespace = &ast.environment;

    // should eventually be one pass with 'lazy' symbol resolution
    let global_context = GlobalContext::new(namespace, input);

    let mut context = Context::new(input);

    let mut constants = Vec::new();
    for (id, term) in &namespace.constants {
        let db_term = parser_term_to_db(term, &mut context, &global_context)?;
        constants.push((id.clone(), db_term)); // CLONE
    }

    let mut inductives = Vec::new();
    for inductive in &namespace.inductives {
        let ty = parser_term_to_db(&inductive.r#type, &mut context, &global_context)?;

        let mut constructors = Vec::new();
        for c in &inductive.constructors {
            constructors.push(ConstructorDB {
                name: c.name.clone(), // CLONE
                r#type: parser_term_to_db(&c.r#type, &mut context, &global_context)?,
            });
        }
        inductives.push(InductiveDB {
            name: inductive.name.clone(), // CLONE
            r#type: ty,
            constructors,
            eliminator: inductive.eliminator.clone(), // CLONE
        });
    }

    Ok(ASTDB {
        environment: NamespaceDB {
            inductives,
            constants,
        },
    })
}

fn parser_term_to_db(
    term: &SourceTerm,
    context: &mut Context,
    global_context: &GlobalContext,
) -> Result<TermDB, DBError> {
    match term {
        SourceTerm::Identifier(id) => {
            if let Some(index) = context.lookup_index(id) {
                Ok(TermDB::Var(index))
            } else if let Some(global_ref) = global_context.resolve(id) {
                Ok(TermDB::GlobalRef(global_ref))
            } else {
                Err(DBError::UnboundIdentifier)
            }
        }

        SourceTerm::Builtin(builtin_type) => Ok(TermDB::Builtin(match builtin_type {
            SourceBuiltinType::Prop => BuiltinTypeDB::Prop,
            SourceBuiltinType::Type(u) => BuiltinTypeDB::Type(*u),
            SourceBuiltinType::U8 => BuiltinTypeDB::U8,
            SourceBuiltinType::Array { dependent } => BuiltinTypeDB::Array {
                dependent: Box::new(parser_term_to_db(dependent, context, global_context)?),
            },
        })),

        SourceTerm::Lambda {
            parameter,
            r#type,
            body,
        } => {
            let db_type = parser_term_to_db(r#type, context, global_context)?;
            context.push(parameter.clone()); // CLONE
            let db_body = parser_term_to_db(body, context, global_context)?;
            context.pop();
            Ok(TermDB::Lambda {
                r#type: Box::new(db_type),
                body: Box::new(db_body),
            })
        }

        SourceTerm::Pi {
            dependent,
            from_type,
            to_type,
        } => {
            let db_from = parser_term_to_db(from_type, context, global_context)?;
            context.push(dependent.clone()); // CLONE
            let db_to = parser_term_to_db(to_type, context, global_context)?;
            context.pop();
            Ok(TermDB::Pi {
                from_type: Box::new(db_from),
                to_type: Box::new(db_to),
            })
        }

        SourceTerm::App { function, argument } => Ok(TermDB::App {
            function: Box::new(parser_term_to_db(function, context, global_context)?),
            argument: Box::new(parser_term_to_db(argument, context, global_context)?),
        }),
    }
}
