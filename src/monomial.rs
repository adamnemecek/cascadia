// the comm and noncomm are distinguished by whether it's ordered or not
// since you need to dinstinguish xxy and xyx and yxx
// i wonder if there's a better way of doing this

// https://github.com/NCAlgebra/NC/tree/54b2a81ebda9e5260328f88f83f56fe8cf472ac3 is the last version to have c++

// albert2005

pub trait Coeff {
    //
}

// pub enum Mono<F> {
//     //
//     Const(F),
//     Mono {
//         //
//     },
// }
use crate::prelude::*;
#[derive(Debug, Clone)]
pub struct Monomial<R: Ring> {
    pub coefficient: R,
    pub exponents: Vec<char>,
}
