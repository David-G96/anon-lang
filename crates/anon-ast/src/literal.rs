use anon_core::interner::Symbol;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Literal {
    String(Symbol),
    Char(char),
    Integer(i64),
    Float(f64),
}
