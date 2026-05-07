use hir::lowering::{
    expr::{Expr, Ref},
    namespace::Namespace,
};

use crate::{
    context::Context,
    kernel::whnf,
    traversal::{shift_indices, substitute},
};

pub fn check_namespace(namespace: &Namespace) {
    for (ind_index, inductive) in namespace.inductives.iter().enumerate() {
        for cons in inductive.constructors.iter() {
            check_inductive_definition(ind_index, &cons.typ);
        }
    }

    for func in namespace.functions.iter() {
        let mut ctx = Context::new();
        check_type(&func.definition, &func.return_type, &mut ctx, namespace);
    }
}

pub fn infer_type(expr: &Expr, ctx: &mut Context, namespace: &Namespace) -> Expr {
    match expr {
        Expr::Var(i) => ctx.lookup(*i).clone(),
        Expr::Ref(r) => infer_global(r, namespace),
        Expr::App { func, arg } => {
            let func_ty = infer_type(func, ctx, namespace);
            let func_ty = whnf(&func_ty, namespace);

            // println!("func_ty: {}", visualizer::pretty(&func_ty));
            // println!("arg: {}", visualizer::pretty(&arg));

            match func_ty {
                Expr::Pi { typ, body } => substitute(&body, &arg, 0),
                _ => panic!("Expected pi type {:?}", func_ty),
            }
        }
        Expr::Lambda { typ, body } => {
            let param_ty = typ
                .as_ref()
                .clone()
                .expect("Cannot infer type of unannotated lambda");
            ctx.extend(param_ty.clone());

            let body_ty = infer_type(body, ctx, namespace);
            ctx.pop();

            Expr::construct_pi(param_ty, body_ty)
        }
        Expr::Match {
            scrutinee,
            branches,
        } => {
            todo!();
            let scrutinee_typ = infer_type(scrutinee, ctx, namespace);
            let scrutinee_typ = whnf(&scrutinee_typ, namespace);

            for (cons_ref, body) in branches {
                let branch_typ =
                    &namespace.inductives[cons_ref.ind].constructors[cons_ref.cons].typ;

                check_type(&body, branch_typ, ctx, namespace);
            }

            todo!()
        }
        _ => todo!(),
    }
}

fn check_type(expr: &Expr, expected: &Expr, ctx: &mut Context, namespace: &Namespace) {
    let inferred = infer_type(expr, ctx, namespace);

    println!("{}", visualizer::pretty(&expr));

    let inferred_nf = whnf(&inferred, namespace);
    let expected_nf = whnf(expected, namespace);

    if inferred_nf != expected_nf {
        panic!(
            "Type mismatch:\n inferred: {:?}\n expected: {:?}",
            inferred_nf, expected_nf
        )
    }
}

fn infer_global(r: &Ref, namespace: &Namespace) -> Expr {
    match r {
        Ref::Fn(i) => namespace.functions[*i].return_type.clone(),
        Ref::Ind(i) => namespace.inductives[*i].typ.clone(),
        Ref::Cons(cons_ref) => namespace.inductives[cons_ref.ind].constructors[cons_ref.cons]
            .typ
            .clone(),
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
            Expr::App { func, arg } => check_argument(func, ind) && check_argument(arg, ind),
            Expr::Pi { typ, body } => {
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
            Expr::Pi { typ, body } => contains_inductive(typ, ind) || contains_inductive(body, ind),
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
    while let Expr::Pi { typ, body } = t {
        if !check_argument(typ, ind) {
            return false;
        }
        t = body;
    }

    returns_inductive(t, ind)
}
