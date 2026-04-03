use hir::lowering::{
    expr::{Expr, Ref},
    program::Program,
};

use crate::traversal::{collect_spine, substitute};

pub fn whnf(term: &Expr, program: &Program) -> Expr {
    match term {
        Expr::App { .. } => beta_reduction(term, program),
        Expr::Match { .. } => iota_reduction(term, program),
        Expr::Ref(g) => delta_reduction(&g, program),
        _ => term.clone(),
    }
}

fn delta_reduction(global_ref: &Ref, program: &Program) -> Expr {
    match global_ref {
        Ref::Fn(index) => program.namespace.functions[*index].wrap_with_lambdas(),
        Ref::Ind(ind) => Expr::Ref(Ref::Ind(*ind)),
        Ref::Cons(cr) => Expr::Ref(Ref::Cons(cr.clone())),
    }
}

fn beta_reduction(term: &Expr, program: &Program) -> Expr {
    if let Expr::App {
        func: function,
        arg: argument,
    } = term
    {
        let f = whnf(function, program);
        let a = whnf(argument, program);

        match f {
            Expr::Lambda { body, .. } => {
                let substituted = substitute(&body, &a, 0);
                whnf(&substituted, program)
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
        let scrut = whnf(scrutinee, program);
        let (head, args) = collect_spine(&scrut);

        match head {
            Expr::Ref(Ref::Cons(r)) => {
                let (_, branch_body) = branches
                    .iter()
                    .find(|(g, _)| *g == r)
                    .expect("Missing branch!");

                let mut result = branch_body.clone();

                for arg in args {
                    result = Expr::construct_application(result, arg);
                }

                whnf(&result, program)
            }
            _ => {
                panic!(
                    "Head in pattern match must be a constructor reference {:?}!",
                    head
                );
            }
        }
    } else {
        term.clone()
    }
}
