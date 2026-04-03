use hir::lowering::{
    expr::{Expr, Ref},
    program::Program,
};

pub fn check_program(program: &Program) {
    for (ind_index, inductive) in program.namespace.inductives.iter().enumerate() {
        for cons in inductive.constructors.iter() {
            check_inductive_definition(ind_index, &cons.typ);
        }
    }
}

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
            Expr::Var(_) | Expr::Builtin(_) => true,

            Expr::App {
                func: function,
                arg: argument,
            } => check_argument(function, ind) && check_argument(argument, ind),

            Expr::Pi { from_type, to_type } => {
                // inductive appearing in domain of an arrow is forbidden
                contains_inductive(from_type, ind) == false && check_argument(to_type, ind)
            }

            _ => true,
        }
    }

    fn contains_inductive(term: &Expr, ind: usize) -> bool {
        match term {
            Expr::App { func, arg } => {
                contains_inductive(func, ind) || contains_inductive(arg, ind)
            }

            Expr::Pi { from_type, to_type } => {
                contains_inductive(from_type, ind) || contains_inductive(to_type, ind)
            }

            _ => false,
        }
    }

    fn returns_inductive(term: &Expr, ind: usize) -> bool {
        match term {
            Expr::App { func, .. } => returns_inductive(func, ind),
            Expr::Pi { to_type, .. } => returns_inductive(to_type, ind),
            Expr::Ref(Ref::Ind(i)) => *i == ind,
            _ => false,
        }
    }

    let mut t = typ;
    while let Expr::Pi { from_type, to_type } = t {
        if !check_argument(from_type, ind) {
            return false;
        }
        t = to_type;
    }

    returns_inductive(t, ind)
}
