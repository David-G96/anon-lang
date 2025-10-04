#![allow(dead_code)]
use std::collections::HashMap;

pub type Sym = StringId;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct StringId(pub(crate) u32);

impl StringId {
    pub(crate) fn from_u32(id: u32) -> Self {
        Self(id)
    }
    pub(crate) fn to_u32(self) -> u32 {
        self.0
    }
}

#[derive(Debug)]
pub struct Interner {
    map: HashMap<String, u32>,
    strings: Vec<String>,
}

impl Interner {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            strings: Vec::new(),
        }
    }

    /// Pre: make sure to check the count in order to prevent overflow
    pub fn intern_or_get(&mut self, s: impl Into<String>) -> Sym {
        let str: String = s.into();
        match self.map.get(&str) {
            // 字符串已经存在，只需要返回
            Some(&idx) => {
                return StringId::from_u32(idx);
            }
            // 字符串尚未内联
            None => {
                let id = self.strings.len();
                self.strings.push(str.clone());
                self.map.insert(str, id as u32);
                return StringId::from_u32(id as u32);
            }
        }
    }

    pub fn resolve(&self, id: Sym) -> Option<&str> {
        self.strings.get(id.to_u32() as usize).map(|x| x.as_str())
    }

    pub fn count(&self) -> u32 {
        (self.strings.len()) as u32
    }
}
