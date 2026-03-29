use hir::lowering::{
    expr::{Expr, Ref},
    program::Program,
};

use crate::traversal::{collect_spine, substitute};

pub fn reduce(term: &Expr, program: &Program) -> Expr {
    match term {
        Expr::App { .. } => beta_reduction(term, program),
        Expr::Match { .. } => iota_reduction(term, program),
        Expr::Ref(g) => delta_reduction(&g, program),
        Expr::Lambda { typ, body } => Expr::construct_binding(*typ.clone(), reduce(body, program)),
        _ => term.clone(),
    }
}

fn delta_reduction(global_ref: &Ref, program: &Program) -> Expr {
    match global_ref {
        Ref::Function(index) => program.namespace.functions[*index].wrap_with_lambdas(),
        Ref::Inductive(ind) => Expr::Ref(Ref::Inductive(*ind)),
        Ref::Constructor(cr) => Expr::Ref(Ref::Constructor(cr.clone())),
        Ref::SelfRef(ind) => Expr::Ref(Ref::Inductive(*ind)),
    }
}

fn beta_reduction(term: &Expr, program: &Program) -> Expr {
    if let Expr::App { function, argument } = term {
        let f = reduce(function, program);
        let a = reduce(argument, program);

        match f {
            Expr::Lambda { body, .. } => {
                let substituted = substitute(&body, &a, 0);
                reduce(&substituted, program)
            }
            _ => Expr::construct_application(f, a),
        }
    } else {
        term.clone()
    }
}

fn iota_reduction(term: &Expr, program: &Program) -> Expr {
    if let Expr::Match {
        scrutinee,
        branches,
    } = term
    {
        let scrut = reduce(scrutinee, program);
        let (head, args) = collect_spine(&scrut);

        match head {
            Expr::Ref(Ref::Constructor(r)) => {
                let (_, branch_body) = branches
                    .iter()
                    .find(|(g, _)| *g == r)
                    .expect("Missing branch!");

                let mut result = branch_body.clone();

                for arg in args {
                    result = Expr::construct_application(result, arg);
                }

                reduce(&result, program)
            }
            _ => {
                panic!("Head in pattern match must be a constructor reference!");
            }
        }
    } else {
        term.clone()
    }
}
