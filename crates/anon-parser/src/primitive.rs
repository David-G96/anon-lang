#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Primitive {
    /// i32
    Int,
    /// f64
    Float,
    /// utf8 encoded character
    Char,
}
