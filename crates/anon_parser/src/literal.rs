use crate::intern::Sym;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Literal {
    String(Sym),
    Char(char),
    Integer(u32),
    Float(f64),
}
