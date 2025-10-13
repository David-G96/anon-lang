use core::panic;
use std::u32;

use string_interner::{StringInterner, symbol::SymbolU32};

pub type SymbolIndex = u32;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol(pub SymbolIndex);

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut interner = Interner::new();

        let test_str1 = "abc";
        let test_str2 = "!@#";
        let test_str3 = "# The Anon programming language

The Anon programming language is designed to be easy and elegant. We put utility and unification first in order to decrease the difficulties to use and remember.

## The category of Anon programming language

To be specific, the anon programming language is a functional, statically typed, memory-automatically managed, out-of-the-box language and a set of tools.

## The features of Anon programming language

* Immutable is better than mutable
* Every effect should be tracked
* Type does not implies the memory layout
* Name is important
* The behaviors of an object defines itself
* We will do want you need to do but you don't want to
* Leave the 1% edge case for us to get 200% of efficiency for you
";

        let sym1 = interner.intern_or_get(test_str1);
        let sym2 = interner.intern_or_get(test_str2);
        let sym3 = interner.intern_or_get(test_str3);

        assert_eq!(interner.resolve(sym1), Some(test_str1));
        assert_eq!(interner.resolve(sym2), Some(test_str2));
        assert_eq!(interner.resolve(sym3), Some(test_str3));
    }
}
