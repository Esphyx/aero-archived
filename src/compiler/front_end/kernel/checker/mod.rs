use super::de_bruijn::{
    builtin::{Builtin, BuiltinType},
    global_ref::GlobalRef,
    program::Program,
    term::Term,
};

pub fn reducer(program: Program) -> Term {
    let term = program.namespace.constants[program.entry_point]
        .definition
        .clone();

    let new_term = reduce(&term, &program);

    new_term
}

fn infer(term: &Term, program: &Program) -> Term {
    todo!()
}

fn reduce(term: &Term, program: &Program) -> Term {
    match term {
        Term::App { function, argument } => {
            let func = reduce(function, program);

            match func {
                Term::Lambda { body, .. } => beta_reduce(&body, &argument, 0),
                Term::Builtin(Builtin::Primitive(_)) => {
                    Term::construct_application(func, reduce(argument, program))
                }
                Term::GlobalRef(_) => unreachable!(), // func should always be delta reduced

                Term::App { .. } => {
                    let reduced_arg = reduce(&argument, program);
                    Term::construct_application(func, reduced_arg)
                }
                _ => panic!(
                    "Left hand side of an application must be a function: {:?}",
                    func
                ),
            }
        }
        Term::GlobalRef(g) => reduce(&delta_reduction(&g, program), program),
        Term::Builtin(_) => term.clone(),
        Term::Lambda { typ, body } => Term::construct_binding(*typ.clone(), reduce(body, program)),
        Term::Var(_) => term.clone(),
        _ => panic!("Unknown case, {:?}", term),
    }
}

fn delta_reduction(global_ref: &GlobalRef, program: &Program) -> Term {
    match global_ref {
        GlobalRef::ConstRef(index) => program.namespace.constants[*index].wrap_with_lambdas(),
        GlobalRef::InductiveRef(_) => todo!(),
    }
}

pub fn beta_reduce(term: &Term, value: &Term, depth: usize) -> Term {
    match term {
        Term::Var(distance) => {
            if *distance == depth {
                // replace with shifted value
                shift(value, 0, depth as isize)
            } else if *distance > depth {
                // free variable decreases by 1 due to lambda being dropped
                Term::Var(distance - 1)
            } else {
                // untouched
                Term::Var(*distance)
            }
        }

        Term::Lambda { typ, body } => {
            let l = typ.as_ref().as_ref().map(|t| beta_reduce(&t, value, depth));
            Term::construct_binding(
                l,
                beta_reduce(body, value, depth + 1), // increase because Var(0) points to lambda argument
            )
        }

        Term::Pi { from_type, to_type } => Term::construct_pi(
            beta_reduce(from_type, value, depth),
            beta_reduce(to_type, value, depth + 1), // increase because Var(0) points to dependent type
        ),
        Term::App { function, argument } => Term::construct_application(
            beta_reduce(function, value, depth),
            beta_reduce(argument, value, depth),
        ),

        Term::GlobalRef(g) => Term::GlobalRef(g.clone()),
        Term::Builtin(b) => Term::Builtin(b.clone()),
    }
}

pub fn shift(term: &Term, cutoff: usize, amount: isize) -> Term {
    match term {
        Term::Var(idx) => {
            if *idx >= cutoff {
                Term::Var((*idx as isize + amount) as usize)
            } else {
                Term::Var(*idx)
            }
        }

        Term::Lambda { typ, body } => {
            let typ = typ.as_ref().as_ref().map(|t| shift(t, cutoff, amount));
            Term::construct_binding(typ, shift(body, cutoff + 1, amount))
        }

        Term::Pi { from_type, to_type } => Term::construct_pi(
            shift(from_type, cutoff, amount),
            shift(to_type, cutoff + 1, amount),
        ),

        Term::App { function, argument } => Term::construct_application(
            shift(function, cutoff, amount),
            shift(argument, cutoff, amount),
        ),

        Term::GlobalRef(g) => Term::GlobalRef(g.clone()),
        Term::Builtin(Builtin::Type(BuiltinType::Array { dependent })) => {
            Term::Builtin(Builtin::Type(BuiltinType::Array {
                dependent: Box::new(shift(dependent, cutoff, amount)),
            }))
        }
        Term::Builtin(builtin) => Term::Builtin(builtin.clone()),
    }
}
