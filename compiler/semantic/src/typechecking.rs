use hir::lowering::{
    de_bruijn,
    expr::{Expr, Ref},
    namespace::Namespace,
};

use crate::{
    context::Context, kernel::whnf, positivity::check_inductive_definition, traversal::substitute,
};

pub fn check_namespace(namespace: &Namespace, de_bruijn_context: &de_bruijn::Context) {
    for (ind_index, inductive) in namespace.inductives.iter().enumerate() {
        for cons in inductive.constructors.iter() {
            check_inductive_definition(ind_index, &cons.typ);
        }
    }

    for func in namespace.functions.iter() {
        let mut ctx = Context::new();
        check_type(
            &func.definition,
            &func.return_type,
            &mut ctx,
            namespace,
            de_bruijn_context,
        );
    }
}

pub fn infer_type(expr: &Expr, ctx: &mut Context, namespace: &Namespace) -> Expr {
    match expr.to_owned() {
        Expr::Var { index, .. } => ctx.lookup(index).clone(),
        Expr::Ref(r) => infer_global(r, namespace),
        Expr::App { func, arg } => {
            let func_ty = infer_type(&func, ctx, namespace);
            let func_ty = whnf(&func_ty, namespace);

            match func_ty {
                Expr::Pi { body, .. } => substitute(&body, &arg, 0),
                _ => panic!("Expected pi type {:?}", func_ty),
            }
        }
        Expr::Lambda { binder, typ, body } => {
            let param_ty = *typ.clone();
            ctx.extend(param_ty.clone());

            let body_ty = infer_type(&body, ctx, namespace);
            ctx.pop();

            Expr::construct_pi(binder, param_ty, body_ty)
        }
        Expr::Match { .. } => todo!(),
        _ => todo!(),
    }
}

fn infer_global(r: Ref, namespace: &Namespace) -> Expr {
    match r {
        Ref::Fn(i) => namespace.functions[i].return_type.clone(),
        Ref::Ind(i) => namespace.inductives[i].typ.clone(),
        Ref::Cons(cons_ref) => namespace.inductives[cons_ref.ind].constructors[cons_ref.cons]
            .typ
            .clone(),
    }
}

fn check_type(
    expr: &Expr,
    expected: &Expr,
    ctx: &mut Context,
    namespace: &Namespace,
    de_bruijn_context: &de_bruijn::Context,
) {
    let inferred = infer_type(expr, ctx, namespace);

    println!("{}", visualizer::pretty(&inferred, de_bruijn_context));

    let inferred_nf = whnf(&inferred, namespace);
    let expected_nf = whnf(expected, namespace);

    println!("{}", visualizer::pretty(&inferred_nf, de_bruijn_context));
    println!("{}", visualizer::pretty(&expected_nf, de_bruijn_context));

    if inferred_nf != expected_nf {
        panic!(
            "Type mismatch:\n inferred: {:?}\n expected: {:?}",
            inferred_nf, expected_nf
        )
    }
}
