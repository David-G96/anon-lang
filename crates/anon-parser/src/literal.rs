#![allow(dead_code)]

use crate::intern::Sym;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Literal {
    String(Sym),
    Char(char),
    Integer(i64),
    Float(f64),
}
