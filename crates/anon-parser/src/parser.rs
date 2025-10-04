use std::{cell::RefCell, rc::Rc};

use crate::{intern::Interner, meta::NoMeta, untyped_ast::Defination};

pub struct Parser {
    source: String,
    interner: Rc<RefCell<Interner>>,
}

impl Parser {
    pub fn new(source: String) -> Self {
        Self {
            source,
            interner: Rc::new(RefCell::new(Interner::new())),
        }
    }

    pub fn parse(&mut self) -> Result<Defination<NoMeta>, String> {
        unimplemented!()
    }
}
