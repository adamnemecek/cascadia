use crate::prelude::*;

pub trait RingClass {
    //
}

use std::ops::{
    Add,
    Mul,
    Sub,
};

pub trait Ring:
    Sized
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Inv<Output = Self>
{
    //
}

// abstract type RingClass {}
pub trait FractionRingClass: RingClass {
    //
}
// trait QuotientRingClass: RingClass {
//
// }
// trait PolyRingClass <: RingClass {}
// trait Ring{T<:RingClass} {}
// trait FractionRing{S<:RingInt,T<:FractionRingClass} <: Ring{T} {}
// trait QuotientRing{S<:RingInt,T<:QuotientRingClass} <: Ring{T} {}
// trait Polynomial{S<:Ring,T<:PolyRingClass} <: Ring{T} {}

pub trait Polynomial<R: Ring>: Ring {
    //
    fn generator(
        &self,
        index: usize,
    ) -> Generator<'_, Self>;
}
// trait RingTrait {}
// trait CommutativeRingTrait <: RingTrait {}
// trait IntegralDomainTrait <: CommutativeRingTrait {}
// trait IntegrallyClosedDomainTrait <: IntegralDomainTrait {}
// trait GCDDomainTrait <: IntegrallyClosedDomainTrait {}
// trait UniqueFactorizationDomainTrait <: GCDDomainTrait {}
// trait PrincipalIdealDomainTrait <: UniqueFactorizationDomainTrait {}
// trait EuclidianDomainTrait <: PrincipalIdealDomainTrait {}
// trait FieldTrait <: EuclidianDomainTrait {}
// trait AlgebraicallyClosedFieldTrait <: FieldTrait {}

macro_rules! ZZ {
    ($ty: ty) => {
        //
    };
}

// Integers
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ZZ(i32);

impl ZZ {
    pub fn mod_(i: i32) -> ZZMod {
        unimplemented!()
    }
}

// impl std::ops::Add for ZZ {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self {
//         Self(self.0 + rhs.0)
//     }
// }

// impl std::ops::Div<usize> for ZZ {
//     type Output = ZZMod;
//     fn div(rhs: usize) -> Self::Output {
//         unimplemented!()
//     }
// }

pub struct Generator<'a, R: Ring> {
    parent: &'a R,
    symbol: &'a str,
}

fn modulo(a: i32, b: i32) -> i32 {
    (a % b + b) % b
}

#[derive(Clone, Copy, PartialEq, Eq)]

pub struct ZZMod1<const M: i32>(i32);

macro_rules! impl_op {
    ($trait: ty, $fn: ident) => {
        impl<const N: i32> $trait for ZZMod1<N> {
            type Output = Self;
            fn $fn(self, rhs: Self) -> Self {
                Self(modulo(self.0.$fn(rhs.0), N))
            }
        }
    };
}

// impl<const N: i32> Add for ZZMod1<N> {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self {
//         // Self(modulo(self.0 + rhs.0, N))
//         // Self(modulo(self.0.add(rhs.0), N))
//     }
// }
impl_op!(Add, add);
impl_op!(Sub, sub);
impl_op!(Mul, mul);

#[derive(Clone, Copy, PartialEq, Eq)]

pub struct ZZMod {
    //
    val: i32,
    mod_: i32,
}

impl ZZMod {
    pub fn new(val: i32, mod_: i32) -> Self {
        Self {
            val: modulo(val, mod_),
            mod_,
        }
    }
}

// impl Add for ZZMod {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self {
//         Self::new()
//     }
// }

// pub struct Frac<

// R = ZZ[x,y]; // polynoimal ring
// S = ZZ[a,b,c]; // polynomial ring
// f = map(R,S,{x^2,x*y,y^2})
//  f(a+b+c^2) =>

pub struct Hom<R: Ring, S: Ring, F: Fn(R) -> S> {
    //
    f: F,
    ph: std::marker::PhantomData<(S, R, F)>,
}

impl<R: Ring, S: Ring, F: Fn(R) -> S> Hom<R, S, F> {
    // fn new()

    pub fn domain(&self) -> R {
        unimplemented!()
    }

    pub fn codomain(&self) -> S {
        unimplemented!()
    }

    pub fn eval(&self, v: R) -> S {
        (self.f)(v)
    }
}

pub struct VectorSpace<R, V> {
    base: V,
    pivr: Vec<R>,
}

mod tests {
    //

    fn test_homomorphism_generic() {
        //
        // ZZ
    }
}
