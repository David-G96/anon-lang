use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use anon_core::{interner::Interner, source::SourceMap};

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
