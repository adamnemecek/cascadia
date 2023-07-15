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

// R = ZZ[x,y];
// S = ZZ[a,b,c];
// f = map(R,S,{x^2,x*y,y^2})
// f(a)

#[derive(Debug, Clone)]
pub struct Monomial<R: Ring + Gens1> {
    pub coefficient: R,
    pub exponents: Vec<char>,
}

impl<R: Ring + Gens1> Monomial<R> {
    pub fn new() -> Self {
        unimplemented!()
    }

    fn substitute(
        &self,
        other: &Self,
        // gens: &Self,
        images: &Self, // in: &[]
    ) -> Self {
        let g1 = self.coefficient.gens();
        let g2 = other.coefficient.gens();
        let mut exponents = vec![];

        for a in self.exponents.iter() {
            let pos =
                g1.iter().position(|x| x == a).unwrap();
            exponents.push(g2[pos]);
        }
        Self {
            coefficient: self.coefficient.clone()
                * other.coefficient.clone(),
            exponents,
        }

        // unimplemented!()
    }
}

pub struct R {
    gens: Vec<char>,
}

mod tests {
    use super::R;
    #[test]
    fn test1() {
        //
        let r1 = R {
            gens: vec!['a', 'b', 'c'],
        };

        let r2 = R {
            gens: vec!['x', 'y'],
        };
    }
}
