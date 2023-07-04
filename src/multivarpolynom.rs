use crate::prelude::*;

pub struct MPoly {
    //
    vars: Vec<String>,
    terms: Vec<Term>,
}

impl MPoly {
    //
    pub fn new() -> Self {
        Self {
            vars: vec![],
            terms: vec![],
        }
    }
}

impl std::ops::Add for MPoly {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        unimplemented!()
    }
}

impl std::ops::Mul for MPoly {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        unimplemented!()
    }
}
