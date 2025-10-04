use string_interner::{StringInterner, symbol::SymbolU32};

pub type SymbolIndex = u32;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol(SymbolIndex);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Interner {
    inner: StringInterner<string_interner::backend::StringBackend>,
}

impl Interner {
    pub fn new() -> Self {
        Self {
            inner: StringInterner::default(),
        }
    }

    pub fn get_or_intern(&mut self, s: impl AsRef<str>) -> Symbol {
        let res: string_interner::symbol::SymbolU32 = self.inner.get_or_intern(s);
        Symbol(string_interner::Symbol::to_usize(res) as SymbolIndex)
    }

    pub fn resolve(&self, sym: Symbol) -> Option<&str> {
        let sym = <SymbolU32 as string_interner::Symbol>::try_from_usize(sym.0 as usize);
        let res = sym.map(|sym| unsafe { self.inner.resolve_unchecked(sym) });
        res
    }
}
