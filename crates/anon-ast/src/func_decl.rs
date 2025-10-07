use anon_core::interner::Symbol;

use crate::expr::Expr;

/// Func decl, without type annotation
#[non_exhaustive]
#[derive(Debug)]
pub struct FuncDecl<M> {
    pub func_name: Symbol,
    pub params: Vec<Symbol>,
    pub func_body: Expr<M>,
}
