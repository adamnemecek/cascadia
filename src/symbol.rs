use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol(usize);

struct Interner {
    data: HashMap<Symbol, String>,
}

impl Interner {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn insert(&mut self, s: &str) -> Symbol {
        unimplemented!()
    }
}
