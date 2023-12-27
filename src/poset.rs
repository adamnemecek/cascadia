// https://doc.sagemath.org/html/en/reference/algebras/sage/combinat/posets/incidence_algebras.html
// see stern1999
// https://github.com/jmichel7/FinitePosets.jl

// antichain https://github.com/IttayWeiss/RustyPosets/blob/main/src/lib.rs#L166

// PosetToTableau
use crate::prelude::*;

// based on sage and that one mathematica package (greene2012?)
// E == element, R == relation
pub struct Poset<'a, E, R: Fn(&E, &E) -> bool> {
    //
    ground_set: &'a [E],
    rel: R,
    // relation_matrix
    // s: std::marker::PhantomData<(E, R)>,
}

// impl<E, L> Poset<E, L> {
//     // based on mathematica hasse diagram ?
//     pub fn new(ground_set: &[E], relations: &[R]) -> Self {
//         unimplemented!()
//     }
// }


// impl<E, R> From<YoungTableau> for Poset<E, R> {
//     // based on mathematica PosetToTableau
//     fn from(value: YoungTableau) -> Self {
//         unimplemented!()
//     }
// }

enum AndiSymmetryStrategy {
    Rank,
    None,
    Digraph,
}

// impl<E, L> Poset<E, L> {
//     //
//     // from macaulay2
//     pub fn new1(data: &[E], relations: &[R]) -> Self {
//         unimplemented!()
//     }
// }
