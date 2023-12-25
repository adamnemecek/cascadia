// https://github.com/emarberg/schurp/blob/master/polynomials.py
// https://github.com/akuczala/coxeter/tree/main/src
// https://github.com/iclue-summer-2020/coxeter
// https://github.com/rgugliel/CoxIter/tree/master
// https://github.com/tscrim/coxeter
// https://github.com/ulthiel/HeckeAlgebras.jl
// https://github.com/tscrim/coxeter
// https://github.com/ulthiel/CoxeterGroups.jl/tree/master

use nalgebra::DMatrix;

// based on https://github.com/punkdit/bruhat/
pub struct CoxeterGroup<T> {
    //
    // m: crate::AbstractMatrix
    gens: Vec<T>,
    rel: DMatrix<T>,
    // rel
    // reduced
    // lookup
    // bruhat
}

impl<T: Eq> CoxeterGroup<T> {
    pub fn new(
        gens: &[T],
        rel: impl Fn(&T, &T) -> Option<usize>,
    ) -> Self {
        let l = gens.len();
        let mut rel_ = DMatrix::from_element(l, l, 2);

        for i in gens.iter() {
            for j in gens.iter() {
                let e: Option<usize> = rel(i, j);
                if let f = rel(j, i) {
                    assert!(e == f);
                } else {
                }
            }
        }
        unimplemented!()

        // Self { gens: vec![] }
    }
}

pub struct CoxeterMatrix {
    //
}

mod tests {
    //
}
