use core::panic;
use std::u32;

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

    pub fn intern_or_get(&mut self, s: impl AsRef<str>) -> Symbol {
        if self.inner.len() >= (u32::MAX - 128) as usize {
            panic!(
                "LOGIC ERROR: Interned String count `{}` is approaching the max capacity of a single String interner, `{}`",
                self.inner.len(),
                u32::MAX
            );
        }
        let res: string_interner::symbol::SymbolU32 = self.inner.get_or_intern(s);
        Symbol(string_interner::Symbol::to_usize(res) as SymbolIndex)
    }

    pub fn resolve(&self, sym: Symbol) -> Option<&str> {
        let inner_sym_result =
            <SymbolU32 as string_interner::Symbol>::try_from_usize(sym.0 as usize);

        match inner_sym_result {
            Some(inner_sym) => {
                debug_assert!(
                    self.inner.resolve(inner_sym).is_some(),
                    "FATAL: A seemingly valid Symbol ID was not found in the Interner."
                );
                Some(unsafe { self.inner.resolve_unchecked(inner_sym) })
            }
            None => {
                panic!(
                    "LOGIC ERROR: Symbol ID value '{}' exceeds the capacity of the inner Interner's Symbol type (SymbolU32).",
                    ""
                );
            }
        }
    }
}
