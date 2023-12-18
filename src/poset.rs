// https://doc.sagemath.org/html/en/reference/algebras/sage/combinat/posets/incidence_algebras.html
// see stern1999
// https://github.com/jmichel7/FinitePosets.jl

// PosetToTableau
use crate::prelude::*;

// based on sage and that one mathematica package
// E == element, R == relation
pub struct Poset<E, R> {
    //
    s: std::marker::PhantomData<(E, R)>,
}

impl<E, L> Poset<E, L> {
    //
    pub fn new(data: &[E], relations: &[R]) -> Self {
        unimplemented!()
    }
}

impl<E, R> From<YoungTableau> for Poset<E, R> {
    fn from(value: YoungTableau) -> Self {
        unimplemented!()
    }
}
