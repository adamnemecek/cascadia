pub trait Inv {
    type Output;
    fn inv(&self) -> Self::Output;
}

pub trait Mod {
    // fn mod_(&)
}

pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait One {
    fn one() -> Self;
    fn is_one(&self) -> bool;
}

impl<T: From<usize> + PartialEq> Zero for T {
    #[inline]
    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }

    #[inline]
    fn zero() -> Self {
        0.into()
    }
}

impl<T: From<usize> + PartialEq> One for T {
    #[inline]
    fn is_one(&self) -> bool {
        self == &Self::one()
    }

    #[inline]
    fn one() -> Self {
        1.into()
    }
}

pub trait DivRem<Rhs = Self>: Sized {
    // type Output;
    // fn div_rem(self, rhs: Rhs) -> Self::Output;
    fn div_rem(self, rhs: Rhs) -> (Self, Self);
}

macro_rules! impl_div_rem {
    ($ty: ty) => {
        impl DivRem for $ty {
            // type Output = (Self, Self);
            fn div_rem(self, rhs: Self) -> (Self, Self) {
                (self / rhs, self % rhs)
            }
        }
    };
}

impl_div_rem!(usize);
impl_div_rem!(isize);
impl_div_rem!(u8);
impl_div_rem!(u16);
impl_div_rem!(u32);

pub fn gcd(a: usize, b: usize) -> usize {
    if b.is_zero() {
        return a;
    }

    // loop {

    // }
    unimplemented!()
}

pub enum Either<A, B> {
    Left(A),
    Both(A, B),
    Right(B),
}

use std::iter::Fuse;
pub struct ZipLongest<A: Iterator, B: Iterator> {
    a: Fuse<A>,
    b: Fuse<B>,
    // len: usize,
}

impl<A: Iterator, B: Iterator> ZipLongest<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self {
            a: a.fuse(),
            b: b.fuse(),
            // len: a.size_hint().0.max(b.size_hint().0),
        }
    }
}

impl<A: Iterator, B: Iterator> Iterator
    for ZipLongest<A, B>
{
    type Item = Either<A::Item, B::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        match (self.a.next(), self.b.next()) {
            (Some(a), (Some(b))) => {
                Either::Both(a, b).into()
            }
            (Some(a), _) => Either::Left(a).into(),
            (_, Some(b)) => Either::Right(b).into(),
            _ => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unimplemented!()
    }
}
