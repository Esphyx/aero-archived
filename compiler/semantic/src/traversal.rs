use hir::lowering::expr::{ConstructorRef, Expr};

pub fn collect_spine(term: &Expr) -> (Expr, Vec<Expr>) {
    let mut args = Vec::new();
    let mut head = term;

    while let Expr::App { function, argument } = head {
        args.push((**argument).clone());
        head = function;
    }

    args.reverse();
    (head.clone(), args)
}

pub fn substitute(term: &Expr, value: &Expr, depth: usize) -> Expr {
    match term {
        Expr::Var(distance) => {
            if *distance == depth {
                // replace with shifted value
                shift_indices(value, 0, depth as isize)
            } else if *distance > depth {
                // free variable decreases by 1 due to lambda being dropped
                Expr::Var(distance - 1)
            } else {
                Expr::Var(*distance)
            }
        }
        Expr::Lambda { typ, body } => {
            let l = typ.as_ref().as_ref().map(|t| substitute(&t, value, depth));
            Expr::construct_binding(
                l,
                substitute(body, value, depth + 1), // increase depth, because Var(0) points to lambda argument
            )
        }
        Expr::Pi { from_type, to_type } => Expr::construct_pi(
            substitute(from_type, value, depth),
            substitute(to_type, value, depth + 1), // increase depth, because Var(0) points to dependent type
        ),
        Expr::App { function, argument } => Expr::construct_application(
            substitute(function, value, depth),
            substitute(argument, value, depth),
        ),
        Expr::Match {
            scrutinee,
            branches,
        } => {
            let scrutinee = Box::new(substitute(scrutinee, value, depth));

            let branches: Vec<(ConstructorRef, Expr)> = branches
                .iter()
                .map(|(ctor, body)| (ctor.clone(), substitute(body, value, depth)))
                .collect();
            Expr::Match {
                scrutinee,
                branches,
            }
        }
        _ => term.clone(),
    }
}

// shifts de Bruijn indices by `amount` starting at `cutoff`
pub fn shift_indices(term: &Expr, cutoff: usize, amount: isize) -> Expr {
    match term {
        Expr::Var(idx) => {
            if *idx >= cutoff {
                Expr::Var((*idx as isize + amount) as usize)
            } else {
                Expr::Var(*idx)
            }
        }

        Expr::Lambda { typ, body } => {
            let typ = typ.as_ref().as_ref().map(|t| shift_indices(t, cutoff, amount));
            Expr::construct_binding(typ, shift_indices(body, cutoff + 1, amount))
        }

        Expr::Pi { from_type, to_type } => Expr::construct_pi(
            shift_indices(from_type, cutoff, amount),
            shift_indices(to_type, cutoff + 1, amount),
        ),

        Expr::App { function, argument } => Expr::construct_application(
            shift_indices(function, cutoff, amount),
            shift_indices(argument, cutoff, amount),
        ),

        Expr::Match {
            scrutinee,
            branches,
        } => Expr::Match {
            scrutinee: Box::new(shift_indices(scrutinee, cutoff, amount)),
            branches: branches
                .iter()
                .map(|(ctor, body)| (ctor.clone(), shift_indices(body, cutoff, amount)))
                .collect(),
        },
        _ => term.clone(),
    }
}
