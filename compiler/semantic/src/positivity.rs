use hir::lowering::expr::{Expr, Ref};

pub fn check_inductive_definition(ind: usize, typ: &Expr) {
    if !is_strictly_positive(ind, typ) {
        panic!(
            "Inductive type {} is not strictly positive at constructors type: {:?}",
            ind, typ
        )
    }
}

pub fn is_strictly_positive(ind: usize, typ: &Expr) -> bool {
    fn check_argument(term: &Expr, ind: usize) -> bool {
        match term {
            Expr::Var { .. } | Expr::Builtin(_) => true,
            Expr::App { func, arg } => check_argument(func, ind) && check_argument(arg, ind),
            Expr::Pi { typ, body, .. } => {
                contains_inductive(typ, ind) == false && check_argument(body, ind)
            }

            _ => true,
        }
    }

    fn contains_inductive(term: &Expr, ind: usize) -> bool {
        match term {
            Expr::App { func, arg } => {
                contains_inductive(func, ind) || contains_inductive(arg, ind)
            }
            Expr::Pi { typ, body, .. } => {
                contains_inductive(typ, ind) || contains_inductive(body, ind)
            }
            _ => false,
        }
    }

    fn returns_inductive(term: &Expr, ind: usize) -> bool {
        match term {
            Expr::App { func, .. } => returns_inductive(func, ind),
            Expr::Pi { body, .. } => returns_inductive(body, ind),
            Expr::Ref(Ref::Ind(i)) => *i == ind,
            _ => false,
        }
    }

    let mut t = typ;
    while let Expr::Pi { typ, body, .. } = t {
        if !check_argument(typ, ind) {
            return false;
        }
        t = body;
    }

    returns_inductive(t, ind)
}
