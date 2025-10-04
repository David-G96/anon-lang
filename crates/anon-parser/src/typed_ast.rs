#![allow(dead_code)]

use crate::{meta::TypeMeta, untyped_ast::{Expr, ExprKind, MatchArm}};


pub type TypedExpr = Expr<TypeMeta>;
pub type TypedExprKind = ExprKind<TypeMeta>;
pub type TypedMatchArm = MatchArm<TypeMeta>;