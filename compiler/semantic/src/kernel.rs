use hir::lowering::{
    expr::{ConsRef, Expr, Ref},
    namespace::Namespace,
};

use crate::traversal::{collect_spine, substitute};

pub fn whnf(term: &Expr, namespace: &Namespace) -> Expr {
    match term {
        Expr::App { func, arg } => beta(func, arg, namespace),
        Expr::Match {
            scrutinee,
            branches,
        } => iota(scrutinee, branches, namespace),
        Expr::Ref(g) => delta(&g, namespace),
        _ => term.clone(),
    }
}

fn beta(func: &Expr, arg: &Expr, namespace: &Namespace) -> Expr {
    match whnf(func, namespace) {
        Expr::Lambda { body, .. } => whnf(&substitute(&body, arg, 0), namespace),
        f => Expr::construct_application(f, arg.clone()),
    }
}

fn iota(scrutinee: &Expr, branches: &[(ConsRef, Expr)], namespace: &Namespace) -> Expr {
    let scrut = whnf(scrutinee, namespace);
    let (head, args) = collect_spine(&scrut);

    let Expr::Ref(Ref::Cons(cons_ref)) = head else {
        panic!(
            "iota: scrutinee does not reduce to a constructor: {:?}",
            head
        )
    };

    let (_, branch_body) = branches
        .iter()
        .find(|(g, _)| *g == cons_ref)
        .expect("Missing branch!");

    let applied = args.into_iter().fold(branch_body.clone(), |acc, arg| {
        Expr::construct_application(acc, arg)
    });

    whnf(&applied, namespace)
}

fn delta(global_ref: &Ref, namespace: &Namespace) -> Expr {
    match global_ref {
        Ref::Fn(index) => namespace.functions[*index].definition.clone(),
        Ref::Ind(ind) => Expr::Ref(Ref::Ind(*ind)),
        Ref::Cons(cr) => Expr::Ref(Ref::Cons(cr.clone())),
    }
}
