use crate::{expr::Expr, pattern::Pattern};

// 模式匹配的单个分支
// 例如: `pattern -> expression`
#[derive(Debug, Clone)]
pub struct MatchArm<M> {
    pub pattern: Pattern,
    pub body: Expr<M>,
}
