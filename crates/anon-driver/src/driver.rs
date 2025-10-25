use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use anon_core::{
    interner::Interner,
    source::{self, SourceMap},
};

pub trait Driver {
    fn interner(&self) -> Ref<'_, Interner>;
    fn interner_mut(&mut self) -> RefMut<'_, Interner>;
    fn source_map(&self) -> &SourceMap;
    fn source_map_mut(&mut self) -> &mut SourceMap;
}

#[allow(dead_code)]
pub struct DebugDriver {
    interner: Rc<RefCell<Interner>>,
    source_map: SourceMap,
}

impl DebugDriver {
    pub fn new() -> Self {
        Self {
            interner: Rc::new(RefCell::new(Interner::new())),
            source_map: SourceMap::new(),
        }
    }
}

impl Driver for DebugDriver {
    fn interner(&self) -> Ref<'_, Interner> {
        self.interner.borrow()
    }

    fn interner_mut(&mut self) -> RefMut<'_, Interner> {
        self.interner.borrow_mut()
    }
    fn source_map(&self) -> &SourceMap {
        &self.source_map
    }

    fn source_map_mut(&mut self) -> &mut SourceMap {
        &mut self.source_map
    }
}

#[cfg(test)]
mod test {
    use anon_compiler::{Lexer, line_tokenizer::PestParser};

    use super::*;
    use crate::driver::DebugDriver;

    #[test]
    fn test_debug_driver() {
        let mut debug_driver = DebugDriver::new();
        let str = "";

        let mut lexer = Lexer::new(str, 4, debug_driver.interner.clone());

    }
}
