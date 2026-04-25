pub mod de_bruijn;
pub mod function;
pub mod inductive;
pub mod expr;
pub mod program;

use de_bruijn::Context;

pub trait Lower<T> {
    fn lower(&self, ctx: &mut Context) -> T;
}
