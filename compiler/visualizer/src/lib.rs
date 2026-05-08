use ast::expression::Binder;
use hir::lowering::{
    de_bruijn::Context,
    expr::{BinderId, Expr},
};

pub struct PrintConfig {
    pub colors: bool,
    pub show_de_bruijn: bool,
    pub show_binder_ids: bool,
    pub show_types: bool,
}

impl Default for PrintConfig {
    fn default() -> Self {
        Self {
            colors: true,
            show_de_bruijn: false,
            show_binder_ids: false,
            show_types: true,
        }
    }
}

struct Printer<'a> {
    output: String,
    context: &'a Context,
    cfg: &'a PrintConfig,
}

impl<'a> Printer<'a> {
    fn new(context: &'a Context, cfg: &'a PrintConfig) -> Self {
        Self {
            output: String::new(),
            context,
            cfg,
        }
    }

    fn keyword(&self, s: &str) -> String {
        if self.cfg.colors {
            format!("\x1b[35m{s}\x1b[0m")
        } else {
            s.to_string()
        }
    }
    fn var(&self, s: &str) -> String {
        if self.cfg.colors {
            format!("\x1b[33m{s}\x1b[0m")
        } else {
            s.to_string()
        }
    }
    fn index(&self, s: &str) -> String {
        if self.cfg.colors {
            format!("\x1b[90m{s}\x1b[0m")
        } else {
            s.to_string()
        }
    }
    fn builtin(&self, s: &str) -> String {
        if self.cfg.colors {
            format!("\x1b[36m{s}\x1b[0m")
        } else {
            s.to_string()
        }
    }
    fn punct(&self, s: &str) -> String {
        if self.cfg.colors {
            format!("\x1b[90m{s}\x1b[0m")
        } else {
            s.to_string()
        }
    }
    fn reference(&self, s: &str) -> String {
        if self.cfg.colors {
            format!("\x1b[32m{s}\x1b[0m")
        } else {
            s.to_string()
        }
    }

    fn print(&mut self, expr: &Expr, indent: usize, new_line: bool) {
        if new_line {
            self.output.push('\n');
            self.output.push_str(&" ".repeat(indent));
        }

        match expr {
            Expr::Var { index, binder } => {
                self.print_var(*index, binder.as_ref());
            }

            Expr::Builtin(builtin) => {
                let s = self.builtin(&format!("{builtin:?}"));
                self.output.push_str(&s);
            }

            Expr::Ref(r) => {
                let s = self.reference(&format!("{r:?}"));
                self.output.push_str(&s);
            }

            Expr::Pi { binder, typ, body } => {
                let s = self.punct("(");
                self.output.push_str(&s);
                self.print_binder_name(binder.as_ref());

                if self.cfg.show_types {
                    let s = self.punct(" : ");
                    self.output.push_str(&s);
                    self.print(typ, indent, false);
                }

                let s = self.keyword(" -> ");
                self.output.push_str(&s);
                self.print(body, indent, false);
                let s = self.punct(")");
                self.output.push_str(&s);
            }

            Expr::Lambda { binder, typ, body } => {
                let s = self.keyword("λ ");
                self.output.push_str(&s);
                self.print_binder_name(binder.as_ref());

                if self.cfg.show_types {
                    let s = self.punct(" : ");
                    self.output.push_str(&s);
                    self.print(typ, indent, false);
                }

                let s = self.punct(",");
                self.output.push_str(&s);
                self.print(body, indent + 4, true);
            }

            Expr::App { func, arg } => {
                let arg_needs_parens = matches!(arg.as_ref(), Expr::App { .. });
                self.print(func, indent, false);
                self.output.push(' ');

                if arg_needs_parens {
                    let s = self.punct("(");
                    self.output.push_str(&s);
                    self.print(arg, indent, false);
                    let s = self.punct(")");
                    self.output.push_str(&s);
                } else {
                    self.print(arg, indent, false);
                }
            }

            Expr::Match {
                scrutinee,
                branches,
            } => {
                let s = self.keyword("match ");
                self.output.push_str(&s);
                self.print(scrutinee, indent, false);
                let s = self.keyword(" with");
                self.output.push_str(&s);

                for (cons, body) in branches.iter() {
                    self.output.push('\n');
                    self.output.push_str(&" ".repeat(indent));
                    let s = self.punct("| ");
                    self.output.push_str(&s);
                    let s = self.reference(&format!("{cons:?}"));
                    self.output.push_str(&s);
                    let s = self.punct(" => ");
                    self.output.push_str(&s);
                    self.print(body, indent + 2, false);
                }
            }
        }
    }

    fn print_var(&mut self, index: usize, binder: Option<&BinderId>) {
        let name = binder
            .and_then(|id| self.context.local.binder_name(id))
            .and_then(|b| {
                if let Binder::Named(n) = b {
                    Some(n.name.clone())
                } else {
                    None
                }
            });

        match name {
            Some(name) => {
                let s = self.var(&name);
                self.output.push_str(&s);
                if self.cfg.show_de_bruijn {
                    let s = self.index(&format!("[{index}]"));
                    self.output.push_str(&s);
                }
                if self.cfg.show_binder_ids {
                    if let Some(id) = binder {
                        let s = self.index(&format!("[#{}]", id.id));
                        self.output.push_str(&s);
                    }
                }
            }
            None => {
                let s = self.index(&format!("{index}"));
                self.output.push_str(&s);
            }
        }
    }

    fn print_binder_name(&mut self, binder: Option<&BinderId>) {
        let name: Option<String> = binder
            .and_then(|id| self.context.local.binder_name(id))
            .and_then(|b| {
                if let Binder::Named(n) = b {
                    Some(n.name.clone())
                } else {
                    None
                }
            });

        let s = self.var(name.as_deref().unwrap_or("_"));
        self.output.push_str(&s);

        if self.cfg.show_binder_ids {
            if let Some(id) = binder {
                let s = self.index(&format!("[#{}]", id.id));
                self.output.push_str(&s);
            }
        }
    }
}

pub fn pretty(expr: &Expr, context: &Context) -> String {
    pretty_with(expr, context, &PrintConfig::default())
}

pub fn pretty_with(expr: &Expr, context: &Context, cfg: &PrintConfig) -> String {
    let mut printer = Printer::new(context, cfg);
    printer.print(expr, 0, false);
    printer.output
}
