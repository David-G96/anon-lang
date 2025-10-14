use anon_ast::expr::Expr;

use crate::untyped_ast::UntypedAST;

pub trait ASTStream: Iterator<Item = Expr<UntypedAST>> {}
