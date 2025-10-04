//! note that this file is named as "types.rs" only because "type" is a reserved word in Rust
//! This file is mainly for the data type called "Type" which represents the type of any object

#![allow(dead_code)]

use std::collections::HashMap;

use crate::{intern::Sym, primitive::Primitive};

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Unit,
    /// primitives, like int, float and char
    Primitive(Primitive),
    ///  Type Var
    Var(Sym),
    /// Sum Type, also known as OR: a + b
    Sum(Box<Self>, Box<Self>),
    /// Product Type, also known as AND: a * b
    Product(Box<Self>, Box<Self>),
    /// named type, most types are unnamed ny default, like tuple
    Named(Sym, Box<Self>),
    /// The arrow type, symboled as "->"
    Arrow(Vec<Predicate>, Box<Self>, Box<Self>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Predicate {
    pub class_name: String,
    pub ty: Type, // 约束作用于的类型，通常是 Variable(String)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Constraint {
    pub name: Sym,
    pub ty: Vec<Type>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeScheme {
    // 保持不变：这些是类型方案中所有自由变量的集合
    // var index -> Sym -> String name
    pub quantified_vars: Vec<Sym>,

    // 一个类型变量可以对应多个约束
    pub constraints: HashMap<usize, Vec<Constraint>>,

    // 实际的类型
    pub ty: Type,
}

#[cfg(test)]
mod test {
    use crate::intern::Interner;

    use super::*;

    #[test]
    fn test_free_type() {
        // int
        let int = Type::Primitive(Primitive::Int);

        // Option<int>
        let option_int = Type::Sum(Box::new(int), Box::new(Type::Unit));

        // Option<A> = A | Unit
        let option_a = Type::Sum(
            Box::new(Type::Var(Interner::new().intern_or_get("A"))),
            Box::new(Type::Unit),
        );

        assert_eq!(option_int, option_int);
        assert_eq!(option_a, option_a);
    }
}
