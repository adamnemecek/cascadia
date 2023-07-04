use crate::prelude::*;

pub struct Term<'a, P: Ring> {
    parent: &'a P,
    base: u64,
    coeff: f64,
}

impl<'a, P: Ring> Term<'a, P> {
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
