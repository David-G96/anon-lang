use anon_core::interner::Symbol;

use crate::{literal::Literal, match_arm::MatchArm};

pub type Sym = Symbol;

#[derive(Debug, Clone)]
pub struct Expr<M> {
    pub meta: M,
    pub kind: ExprKind<M>,
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum ExprKind<M> {
    Object(Sym),
    //  => Apply()
    Application {
        func: Box<Self>,
        args: Vec<Self>,
    },
    Literal(Literal),
    If {
        condition: Box<Self>,
        consequence: Box<Self>,
        alternative: Box<Self>,
    },

    Match {
        // 待匹配的值，例如: `match val with ...` 中的 `val`
        value: Box<ExprKind<M>>,
        // 匹配分支列表
        arms: Vec<MatchArm<M>>,
    },
}
