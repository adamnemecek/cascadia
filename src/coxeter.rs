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
        let mut rel_ =
            DMatrix::from_element(l, l, 2.into());

        for i in gens.iter() {
            for j in gens.iter() {
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

pub struct CoxeterMatrix {
    //
}

mod tests {
    //
}
