use hir::lowering::expr::Expr;

pub fn pretty(expr: &Expr) -> String {
    let mut output = String::new();
    print_indent(&mut output, expr, 0, false);
    output
}

pub fn print_indent(output: &mut String, expr: &Expr, indent: usize, new_line: bool) {
    if new_line {
        output.push('\n');
        output.push_str(&" ".repeat(indent));
    }

    match expr {
        Expr::Var(i) => output.push_str(&format!("{}", i)),
        Expr::Builtin(builtin) => output.push_str(&format!("{:?}", builtin)),
        Expr::Ref(r) => output.push_str(&format!("{:?}", r)),
        Expr::Pi { typ, body } => {
            output.push_str("(");
            print_indent(output, typ, indent, false);
            output.push_str(" -> ");
            print_indent(output, body, indent, false);
            output.push_str(")");
        }
        Expr::Lambda { typ, body } => {
            output.push_str("λ(");
            print_indent(output, &typ.clone().unwrap(), 0, false);
            output.push_str("), ");
            print_indent(output, body, indent + 4, true);
        }
        Expr::App { func, arg } => {
            print_indent(output, func, indent, false);
            output.push(' ');
            print_indent(output, arg, indent, false);
        }
        Expr::Match {
            scrutinee,
            branches,
        } => {
            output.push_str("match ");
            print_indent(output, scrutinee, indent, false);
            output.push_str(" with");
            for (cons, body) in branches.iter() {
                output.push_str(&format!("\n{}| {:?} => ", " ".repeat(indent), cons));
                print_indent(output, body, indent, false);
            }
        }
    }
}
