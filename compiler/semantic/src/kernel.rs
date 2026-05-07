use hir::lowering::{
    expr::{Expr, Ref},
    namespace::Namespace,
};

use crate::traversal::{collect_spine, substitute};

pub fn whnf(term: &Expr, namespace: &Namespace) -> Expr {
    match term {
        Expr::App { .. } => beta_reduction(term, namespace),
        Expr::Match { .. } => iota_reduction(term, namespace),
        Expr::Ref(g) => delta_reduction(&g, namespace),
        _ => term.clone(),
    }
}

fn delta_reduction(global_ref: &Ref, namespace: &Namespace) -> Expr {
    match global_ref {
        Ref::Fn(index) => namespace.functions[*index].definition.clone(),
        Ref::Ind(ind) => Expr::Ref(Ref::Ind(*ind)),
        Ref::Cons(cr) => Expr::Ref(Ref::Cons(cr.clone())),
    }
}

fn beta_reduction(term: &Expr, namespace: &Namespace) -> Expr {
    if let Expr::App { func, arg } = term {
        let f = whnf(func, namespace);
        let a = whnf(arg, namespace);

        match f {
            Expr::Lambda { body, .. } => {
                let substituted = substitute(&body, &a, 0);
                whnf(&substituted, namespace)
            }
            _ => Expr::construct_application(f, a),
        }
    } else {
        term.clone()
    }
}

fn iota_reduction(term: &Expr, namespace: &Namespace) -> Expr {
    if let Expr::Match {
        scrutinee,
        branches,
    } = term
    {
        let scrut = whnf(scrutinee, namespace);
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

                whnf(&result, namespace)
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
