
use crate::{intern::Sym, literal::Literal, pattern::Pattern};

/// Top level decl
#[non_exhaustive]
#[derive(Debug)]
pub enum Defination<M> {
    FuncDecl(FuncDecl<M>),
}

/// Func decl, without type annotation
#[non_exhaustive]
#[derive(Debug)]
pub struct FuncDecl<M> {
    pub func_name: Sym,
    pub params: Vec<Sym>,
    pub func_body: ExprKind<M>,
}

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

// 模式匹配的单个分支
// 例如: `pattern -> expression`
#[derive(Debug, Clone)]
pub struct MatchArm<M> {
    pub pattern: Pattern,
    pub body: Expr<M>,
}

#[cfg(test)]
mod test {
    use crate::intern::Interner;
    use crate::meta::NoMeta;

    use super::*;
    #[test]
    fn test_top_level() {
        // e.g.
        // func_a = 1
        let mut intern = Interner::new();
        let func_a_name = "func_a";
        let func_a_name_sym = intern.intern_or_get(func_a_name);
        let expr_a_kind = ExprKind::<NoMeta>::Literal(Literal::Integer(1));
        let func_a_body = Expr::<NoMeta> {
            meta: NoMeta,
            kind: expr_a_kind.clone(),
        };
        let func_a_decl = FuncDecl::<NoMeta> {
            func_name: func_a_name_sym,
            params: vec![],
            func_body: func_a_body.kind,
        };

        // e.g.
        // func_b x = match x
        //      case 0 -> func_a
        //      case _ -> 0
        let func_b_name = "func_b";
        let func_b_name_sym = intern.intern_or_get(func_b_name);
        let expr_b_kind = ExprKind::<NoMeta>::Match {
            value: Box::new(ExprKind::Object(intern.intern_or_get("x"))),
            arms: vec![
                MatchArm::<NoMeta> {
                    pattern: Pattern::Literal(Literal::Integer(0)),
                    body: Expr {
                        meta: NoMeta,
                        kind: expr_a_kind.clone(),
                    },
                },
                MatchArm::<NoMeta> {
                    pattern: Pattern::Wildcard,
                    body: Expr {
                        meta: NoMeta,
                        kind: ExprKind::Literal(Literal::Integer(1)),
                    },
                },
            ],
        };
        let func_b_body = Expr::<NoMeta> {
            meta: NoMeta,
            kind: expr_b_kind,
        };
        let func_b_decl = FuncDecl::<NoMeta> {
            func_name: func_b_name_sym,
            params: vec![intern.intern_or_get("x")],
            func_body: func_b_body.kind,
        };
    }
}
