use crate::{intern::Sym, literal::Literal, pattern::Pattern};

/// top level decl
#[non_exhaustive]
#[derive(Debug)]
pub enum Defination {
    FuncDecl(FuncDecl),
}

/// func decl, without type annotation
#[non_exhaustive]
#[derive(Debug)]
pub struct FuncDecl {
    func_name: Sym,
    params: Vec<Sym>,
    func_body: Expr,
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Expr {
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
        value: Box<Expr>,
        // 匹配分支列表
        arms: Vec<MatchArm>,
    },
}

// 模式匹配的单个分支
// 例如: `pattern -> expression`
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Expr,
}

#[cfg(test)]
mod test {
    use crate::intern::Interner;

    use super::*;

    fn test_top_level() {
        let mut intern = Interner::new();
        let func_name = "func_a";
        let func_name_sym = intern.intern_or_get(func_name);
    }
}
