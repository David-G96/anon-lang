use anon_ast::expr::Expr;

use crate::untyped_ast::UntypedAST;

pub trait UntypedASTStream: Iterator<Item = Expr<UntypedAST>> {}
