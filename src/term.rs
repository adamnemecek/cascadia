use crate::prelude::*;

pub struct TermRef<'a, P: Ring> {
    parent: &'a P,
    base: u64,
    coeff: f64,
}

impl<'a, P: Ring> TermRef<'a, P> {
    pub fn new(
        parent: &'a P,
        base: u64,
        coeff: f64,
    ) -> Self {
        Self {
            parent,
            base,
            coeff,
        }
    }
}

pub struct Term {
    base: u64,
    coeff: f64,
}

impl Term {
    pub fn new(base: u64, coeff: f64) -> Self {
        Self { base, coeff }
    }
}
