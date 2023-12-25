// https://github.com/emarberg/schurp/blob/master/polynomials.py
// https://github.com/akuczala/coxeter/tree/main/src
// https://github.com/iclue-summer-2020/coxeter
// https://github.com/rgugliel/CoxIter/tree/master
// https://github.com/tscrim/coxeter
// https://github.com/ulthiel/HeckeAlgebras.jl
// https://github.com/tscrim/coxeter
// https://github.com/ulthiel/CoxeterGroups.jl/tree/master

// based on https://github.com/punkdit/bruhat/
pub struct CoxeterGroup<T> {
    //
    // m: crate::AbstractMatrix
    gens: Vec<T>,
}

impl<T> CoxeterGroup<T> {
    pub fn new(
        gens: &[T],
        rel: impl Fn(usize, usize) -> Option<T>,
    ) -> Self {
        unimplemented!()
    }
}

pub struct CoxeterMatrix {
    //
}
