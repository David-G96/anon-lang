use anon_ast::expr::Sym;

#[derive(Debug)]
pub enum BuiltIn {
    Int,
    Float,
    Char,
    Array,
}

#[derive(Debug)]
pub enum Types {
    Unit,
    BuiltIn(BuiltIn),
    Arrow(Box<Self>, Box<Self>),
    Sum(Vec<Self>),
    Product(Vec<Self>),
    Named(Sym, Box<Self>),
}

#[derive(Debug)]
pub enum MemTypes {
    // platform independent
    U8,
    U16,
    U32,
    U64,
    I32,
    I64,
    F32,
    F64,
    // platform depending
    Sized,
    Ptr,
}
