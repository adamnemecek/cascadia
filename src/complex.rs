// use crate::{Zero, One};
// https://github.com/rust-num/num-complex/blob/master/src/lib.rs

use crate::prelude::*;

pub struct Complex<R> {
    pub re: R,
    pub im: R,
}

impl<R> Complex<R> {
    //

    pub fn new(re: R, im: R) -> Self {
        Self { re, im }
    }
    pub fn arg(&self) {
        unimplemented!()
    }

    pub fn exp(&self) -> Self {
        unimplemented!()
    }

    // pub fn re(&self) -> R {
    //     self.re
    // }

    pub fn sign(&self) -> Sign {
        // (Self::i * self.arg()).exp()
        unimplemented!()
    }
}

// impl <R: Zero + One>  Complex<R> {
//     fn I() -> Self {
//         Self::new(R::zero(), R::one())
//     }
// }

impl<R: std::ops::Add<Output = R>> std::ops::Add
    for Complex<R>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl<R: std::ops::Sub<Output = R>> std::ops::Sub
    for Complex<R>
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl<R> std::ops::Mul for Complex<R> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        unimplemented!()
    }
}

impl<R> std::ops::Div for Complex<R> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        unimplemented!()
    }
}
