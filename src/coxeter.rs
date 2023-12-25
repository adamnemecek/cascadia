// https://github.com/emarberg/schurp/blob/master/polynomials.py
// https://github.com/akuczala/coxeter/tree/main/src
// https://github.com/iclue-summer-2020/coxeter
// https://github.com/rgugliel/CoxIter/tree/master
// https://github.com/tscrim/coxeter
// https://github.com/ulthiel/HeckeAlgebras.jl
// https://github.com/tscrim/coxeter
// https://github.com/ulthiel/CoxeterGroups.jl/tree/master

use nalgebra::DMatrix;

use crate::prelude::*;

// based on https://github.com/punkdit/bruhat/
pub struct CoxeterGroup<T> {
    //
    // m: crate::AbstractMatrix
    gens: Vec<T>,
    // conatural?
    rel: DMatrix<CoNat<usize>>,
    // rel
    // reduced
    // lookup
    // bruhat
}

fn is_symmetric<T>(m: &DMatrix<T>) -> bool {
    unimplemented!();
}

impl<T: Eq + Clone> CoxeterGroup<T> {
    pub fn new(
        gens: &[T],
        rel: impl Fn(&T, &T) -> Option<CoNat<usize>>,
    ) -> Self {
        let l = gens.len();

        let cotwo: CoNat<_> = 2.into();

        let mut rel_ = DMatrix::from_element(l, l, cotwo);

        for i in gens.iter() {
            for j in gens.iter() {
                // diagonal has to be one or more
                // others have to be two or more
                let e = rel(i, j);
                if let f = rel(j, i) {
                    assert!(e == f);
                } else {
                    //
                }
            }
        }

        is_symmetric(&rel_);

        Self {
            gens: gens.to_vec(),
            rel: rel_,
        }
    }
}

impl<T: Eq + Clone> PartialEq for CoxeterGroup<T> {
    fn eq(&self, other: &Self) -> bool {
        self == other || false
    }
}

pub struct CoxeterMatrix {
    //
}

mod tests {
    //
}
