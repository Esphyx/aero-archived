use super::super::grammar::ast::parameter::Parameter;

use super::{
    super::grammar::ast::{SourceAST, function::SourceFunction, term::SourceTerm},
    de_bruijn::{
        builtin::{Builtin, BuiltinType},
        constant::Function,
        context::{BuiltinContext, GlobalContext, LocalContext},
        inductive::{Constructor, Inductive},
        namespace::Namespace,
        program::Program,
        term::Term,
    },
};

pub mod builtin;
pub mod constant;
pub mod context;
pub mod global_ref;
pub mod inductive;
pub mod namespace;
pub mod program;
pub mod term;

const ENTRY_POINT_NAME: &str = "main";
pub fn ast_to_de_bruijn<'a>(ast: &'a SourceAST) -> Program {
    let namespace = &ast.namespace;

    // should eventually be one pass with 'lazy' symbol resolution
    let global_context = GlobalContext::new(namespace);
    let builtin_context = BuiltinContext {};

    let mut local_context = LocalContext::new();

    let mut entry_point = None;
    let mut constants = Vec::new();
    for (i, function) in namespace.constants.iter().enumerate() {
        let SourceFunction {
            name,
            parameters,
            return_type,
            body,
        } = function;

        if name.get_name_str() == ENTRY_POINT_NAME {
            if !parameters.is_empty() {
                panic!("Main function musn't have arguments!");
            }
            entry_point = Some(i);
        }

        let mut parameter_types = Vec::new();
        for Parameter { name, typ } in parameters.iter() {
            parameter_types.push(term_to_de_bruijn(
                typ,
                &mut local_context,
                &global_context,
                &builtin_context,
            ));

            local_context.push(name.clone());
        }

        let definition =
            term_to_de_bruijn(body, &mut local_context, &global_context, &builtin_context);

        for _ in 0..parameters.len() {
            local_context.pop();
        }

        let return_type = return_type
            .as_ref()
            .map(|r| term_to_de_bruijn(&r, &mut local_context, &global_context, &builtin_context));

        constants.push(Function {
            name: name.clone(),
            return_type,
            parameter_types,
            definition,
        });
    }

    let mut inductives = Vec::new();
    for source_inductive in &namespace.inductives {
        let ty = term_to_de_bruijn(
            &source_inductive.typ,
            &mut local_context,
            &global_context,
            &builtin_context,
        );

        let mut constructors = Vec::new();
        for c in &source_inductive.constructors {
            constructors.push(Constructor {
                name: c.name.clone(),
                typ: term_to_de_bruijn(
                    &c.typ,
                    &mut local_context,
                    &global_context,
                    &builtin_context,
                ),
            });
        }

        inductives.push(Inductive {
            name: source_inductive.name.clone(),
            typ: ty,
            constructors,
            eliminator: source_inductive.eliminator.clone(),
        });
    }

    if let Some(entry_point) = entry_point {
        Program::from(((inductives, constants), entry_point))
    } else {
        panic!("No entry point!")
    }
}

fn term_to_de_bruijn<'a>(
    term: &'a SourceTerm,
    local_context: &mut LocalContext,
    global_context: &GlobalContext,
    builtin_context: &BuiltinContext,
) -> Term {
    match term {
        SourceTerm::Identifier(id) => {
            if let Some(index) = local_context.lookup_index(id) {
                Term::Var(index)
            } else if let Some(builtin) = builtin_context.resolve(id) {
                Term::Builtin(builtin)
            } else if let Some(global_ref) = global_context.resolve(id) {
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
                .map(|t| term_to_de_bruijn(&t, local_context, global_context, builtin_context));

            local_context.push(name.clone());

            let body = term_to_de_bruijn(body, local_context, global_context, builtin_context);

            local_context.pop();

            Term::construct_application(
                Term::construct_binding(typ, body),
                term_to_de_bruijn(value, local_context, global_context, builtin_context),
            )
        }

        SourceTerm::Builtin(builtin_source) => {
            Term::Builtin(Builtin::Type(BuiltinType::from_source(
                builtin_source,
                local_context,
                global_context,
                builtin_context,
            )))
        }

        // SourceTerm::Lambda {
        //     name: parameter_name,
        //     type_specifier: parameter_type,
        //     body,
        // } => {
        //     let new_type = term_to_de_bruijn(
        //         parameter_type,
        //         local_context,
        //         global_context,
        //         builtin_context,
        //     );
        //     local_context.push(parameter_name.clone());
        //     let new_body = term_to_de_bruijn(body, local_context, global_context, builtin_context);
        //     local_context.pop();
        //     Term::Lambda {
        //         typ: Box::new(new_type),
        //         body: Box::new(new_body),
        //     }
        // }
        SourceTerm::Arrow {
            dependent,
            from_type,
            to_type,
        } => {
            let new_from_type =
                term_to_de_bruijn(from_type, local_context, global_context, builtin_context);

            if let Some(name) = dependent {
                local_context.push(name.clone());
            }
            let new_to_type =
                term_to_de_bruijn(to_type, local_context, global_context, builtin_context);

            if let Some(_) = dependent {
                local_context.pop();
            }

            Term::Pi {
                from_type: Box::new(new_from_type),
                to_type: Box::new(new_to_type),
            }
        }

        SourceTerm::App { function, argument } => Term::App {
            function: Box::new(term_to_de_bruijn(
                function,
                local_context,
                global_context,
                builtin_context,
            )),
            argument: Box::new(term_to_de_bruijn(
                argument,
                local_context,
                global_context,
                builtin_context,
            )),
        },
    }
}
