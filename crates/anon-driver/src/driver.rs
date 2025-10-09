use std::{cell::RefCell, rc::Rc};

use anon_core::interner::Interner;

pub struct Driver {
    interner: Rc<RefCell<Interner>>,
}

impl Driver {
    pub fn new(config: ()) {}

    pub fn load_source() {}

    pub fn run() {}

    pub fn parse() {}
}

pub struct DebugDriver {
    interner: Rc<RefCell<Interner>>,
}
