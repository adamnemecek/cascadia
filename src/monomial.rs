// the comm and noncomm are distinguished by whether it's ordered or not
// since you need to dinstinguish xxy and xyx and yxx
// i wonder if there's a better way of doing this

// https://github.com/NCAlgebra/NC/tree/54b2a81ebda9e5260328f88f83f56fe8cf472ac3 is the last version to have c++

// albert2005

use crate::prelude::*;

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

// R = ZZ[x,y];
// S = ZZ[a,b,c];
// f = map(R,S,{x^2,x*y,y^2})
// f(a)

// assert(#images == #(gens S))

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZZ1(usize);

impl std::ops::Add for ZZ1 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for ZZ1 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Mul for ZZ1 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl Zero for ZZ1 {
    fn zero() -> Self {
        Self(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

#[derive(Debug, Clone)]
pub struct Monomial<R: Ring + Gens1> {
    pub coefficient: R,
    pub exponents: Vec<char>,
}

impl<R: Ring + Gens1> Monomial<R> {
    fn new(coefficient: R, exponents: Vec<char>) -> Self {
        Self {
            coefficient,
            exponents,
        }
    }

    pub fn eval(
        &self,
        f: impl Fn(char) -> Monomial<R>,
    ) -> Vec<Self> {
        let c = &self.coefficient;
        self.exponents
            .iter()
            .map(|e| f(*e) * c.clone())
            .collect()
        // unimplemented!()
    }
}

impl<R: Ring + Gens1> std::ops::Mul for Monomial<R> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(
            self.coefficient * rhs.coefficient,
            self.exponents
                .iter()
                .cloned()
                .chain(rhs.exponents.iter().cloned())
                .collect(),
        )
    }
}

impl<R: Ring + Gens1> std::ops::Mul<R> for Monomial<R> {
    type Output = Self;
    fn mul(self, rhs: R) -> Self {
        Self::new(self.coefficient * rhs, self.exponents)
    }
}

// fn substitute(
//     &self,
//     other: &Self,
//     // gens: &Self,
//     images: &Self, // in: &[]
// ) -> Self {
//     let g1 = self.coefficient.gens();
//     let g2 = other.coefficient.gens();
//     let mut exponents = vec![];

//     for a in self.exponents.iter() {
//         let pos =
//             g1.iter().position(|x| x == a).unwrap();
//         exponents.push(g2[pos]);
//     }
//     Self {
//         coefficient: self.coefficient.clone()
//             * other.coefficient.clone(),
//         exponents,
//     }

//     // unimplemented!()
// }

// Self {
//     coefficient: self.coefficient.clone()
//         * other.coefficient.clone(),
//     exponents,
// }
pub fn hom(
    g1: &[char],
    g2: &[char],
    images: &[char],
) -> Vec<char> {
    //

    let mut out = vec![];
    for a in images.iter() {
        let pos = g1.iter().position(|x| x == a).unwrap();
        out.push(g2[pos]);
    }

    out
}

// pub fn hom(
//     g1: &[char],
//     g2: &[char],
//     images: &[char],
// ) -> Vec<char> {
//     //

//     let mut out = vec![];
//     for a in images.iter() {
//         let pos = g1.iter().position(|x| x == a).unwrap();
//         out.push(g2[pos]);
//     }

//     out
// }

pub struct R {
    gens: Vec<char>,
}

mod tests {
    use super::{
        hom,
        Monomial,
        R,
    };
    #[test]
    fn test1() {
        //
        // let r1 = R {
        //     gens: vec!['a', 'b', 'c'],
        // };

        // let r2 = R {
        //     gens: vec!['x', 'y'],
        // };
        // g1 should be just
        let a = hom(&['x', 'y'], &['a'], &['c']);

        println!("{:?}", a);
    }

    #[test]
    fn test2() {
        // Monomial::
    }
}
