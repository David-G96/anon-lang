use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anon_core::interner::Symbol;

/// sig for func
/// e.g map :: (a->b) -> [a] -> [b]
/// print :: Show a => a -> IO ()
#[derive(Debug)]
pub struct Sig {
    name: Symbol,
    constraints: HashMap<Symbol, HashSet<Symbol>>,
    arrow: Vec<Symbol>,
}

#[derive(Debug)]
pub enum ParseSigError {}

impl FromStr for Sig {
    type Err = ParseSigError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}
