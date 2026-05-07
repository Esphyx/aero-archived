pub mod de_bruijn;
pub mod expr;
pub mod function;
pub mod inductive;
pub mod namespace;

use de_bruijn::Context;

pub trait Lower<T> {
    fn lower(&self, ctx: &mut Context) -> T;
}
