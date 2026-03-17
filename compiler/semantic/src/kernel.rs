use hir::lowering::{program::Program, term::{Builtin, BuiltinType, GlobalRef, Term}};

pub fn reducer(program: Program) -> Term {
    for (index, inductive) in program.namespace.inductives.iter().enumerate() {
        for cons in inductive.constructors.iter() {
            check_inductive_definition(index, &cons.typ);
        }
    }

    let term = program.namespace.functions[program.entry_point]
        .definition
        .clone();

    let new_term = reduce(&term, &program);

    new_term
}

fn check_inductive_definition(ind: usize, typ: &Term) {
    if !is_strictly_positive(ind, typ) {
        panic!("Inductive type {} is not strictly positive", ind)
    }
}

fn is_strictly_positive(ind: usize, typ: &Term) -> bool {
    fn check_argument(term: &Term, ind: usize) -> bool {
        match term {
            Term::GlobalRef(GlobalRef::SelfRef(_)) => true,

            Term::Var(_) | Term::Builtin(_) => true,

            Term::App { function, argument } => {
                check_argument(function, ind) && check_argument(argument, ind)
            }

            Term::Pi { from_type, to_type } => {
                // inductive appearing in domain of an arrow is forbidden
                contains_inductive(from_type, ind) == false && check_argument(to_type, ind)
            }

            _ => true,
        }
    }

    fn contains_inductive(term: &Term, ind: usize) -> bool {
        match term {
            Term::GlobalRef(GlobalRef::SelfRef(i)) => *i == ind,

            Term::App { function, argument } => {
                contains_inductive(function, ind) || contains_inductive(argument, ind)
            }

            Term::Pi { from_type, to_type } => {
                contains_inductive(from_type, ind) || contains_inductive(to_type, ind)
            }

            _ => false,
        }
    }

    fn returns_inductive(term: &Term, ind: usize) -> bool {
        match term {
            Term::GlobalRef(GlobalRef::SelfRef(i)) => *i == ind,
            Term::App { function, .. } => returns_inductive(function, ind),
            Term::Pi { to_type, .. } => returns_inductive(to_type, ind),
            _ => false,
        }
    }

    let mut t = typ;

    while let Term::Pi { from_type, to_type } = t {
        if !check_argument(from_type, ind) {
            return false;
        }
        t = to_type;
    }

    returns_inductive(t, ind)
}

fn collect_spine(term: &Term) -> (Term, Vec<Term>) {
    let mut args = Vec::new();
    let mut head = term;

    while let Term::App { function, argument } = head {
        args.push((**argument).clone());
        head = function;
    }

    args.reverse();
    (head.clone(), args)
}

fn delta_reduction(global_ref: &GlobalRef, program: &Program) -> Term {
    match global_ref {
        GlobalRef::ConstRef(index) => program.namespace.functions[*index].wrap_with_lambdas(),
        GlobalRef::InductiveRef(ind) => Term::GlobalRef(GlobalRef::InductiveRef(*ind)),
        GlobalRef::ConstructorRef {
            inductive,
            constructor,
        } => Term::GlobalRef(GlobalRef::ConstructorRef {
            inductive: *inductive,
            constructor: *constructor,
        }),
        GlobalRef::EliminatorRef(_) => todo!(),
        GlobalRef::SelfRef(ind) => Term::GlobalRef(GlobalRef::InductiveRef(*ind)),
    }
}

fn iota_reduction(ind: usize, args: Vec<&Term>) -> Term {
    todo!()
}

fn reduce(term: &Term, program: &Program) -> Term {
    match term {
        Term::App { function, argument } => {
            let func = reduce(function, program);

            match func {
                Term::Lambda { body, .. } => beta_reduce(&body, &argument, 0),
                Term::Builtin(Builtin::Primitive(_)) => {
                    // TODO: replace with special form for builtin application for later code generation
                    Term::construct_application(func, reduce(argument, program))
                }
                Term::GlobalRef(GlobalRef::EliminatorRef(ind)) => {
                    let apps_count = program.namespace.inductives[ind].constructors.len() + 1;

                    todo!();

                    // iota_reduction(ind, argument);
                }
                Term::GlobalRef(_) => unreachable!(),

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
        Term::Lambda { typ, body } => Term::construct_binding(*typ.clone(), reduce(body, program)),
        _ => term.clone(),
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
        _ => term.clone(),
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
        Term::Builtin(Builtin::Type(BuiltinType::Array { dependent })) => {
            Term::Builtin(Builtin::Type(BuiltinType::Array {
                dependent: Box::new(shift(dependent, cutoff, amount)),
            }))
        }
        _ => term.clone(),
    }
}
