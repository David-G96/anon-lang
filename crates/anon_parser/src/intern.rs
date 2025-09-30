#[allow(dead_code)]
use std::collections::HashMap;
pub type Sym = u32;

#[derive(Debug)]
pub struct Interner {
    map: HashMap<String, usize>,
    str_pool: Vec<String>,
}

impl Interner {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            str_pool: Vec::new(),
        }
    }

    pub fn intern_or_get(&mut self, s: impl Into<String>) -> Sym {
        let str: String = s.into();
        match self.map.get(&str) {
            // 字符串已经存在，只需要返回
            Some(&idx) => {
                return idx as Sym;
            }
            // 字符串尚未内联
            None => {
                let id = self.str_pool.len();
                self.str_pool.push(str.clone());
                self.map.insert(str, id);
                return id as u32;
            }
        }
    }

    pub fn resolve(&self, id: Sym) -> Option<&str> {
        self.str_pool.get(id as usize).map(|x| x.as_str())
    }

    pub fn count(&self) -> u32 {
        (self.str_pool.len()) as u32
    }
}
