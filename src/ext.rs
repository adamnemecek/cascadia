pub trait NCRing3Elem<const N: usize, R: NCRing3<N>> {
    //
}
pub trait NCRing3<const N: usize>: Sized {
    type Elem: NCRing3Elem<N, Self>;
}

pub trait Ring3Elem<const N: usize, R: Ring3<N>> {
    //
}
pub trait Ring3<const N: usize>: Sized {
    type Elem: Ring3Elem<N, Self>;
}

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

pub trait Ring1 {
    type Elem;
}

struct ZZ {}

pub trait RingElem<const N: usize, R: Ring2<N>>:
    Sized
{
    //add, mul etc
}

macro_rules! expr {
    () => {
        //
    };
}

impl<const N: usize, R: Ring2<N>> Expr<N, R> {
    fn eval(self) {
        //
    }
}

// impl<const N: usize, R: Ring2<N>, E: RingElem<N, R>> From<E>
//     for Expr<N, R>
// {
//     fn from(value: R::Elem) -> Self {
//         unimplemented!()
//     }
// }

impl Ring1 for ZZ {
    //
    type Elem = usize;
}

// creates a polynomial ring over ring R with the symbols
pub fn poly<R: Ring1, const N: usize>(
    r: R,
    symbols: [&str; N],
) -> (usize, [R::Elem; N]) {
    //
    // [0; N]
    unimplemented!()
}

fn poly_test() {
    let (P, [b, c]) = poly(ZZ {}, ["s", "d"]);
}

// pub fn hom<R: Ring1, S: Ring1>(s: S, r: R, images: []) {
//
// }

pub struct Gens<const N: usize, R: Ring1>([R::Elem; N]);

// FromIterator<term::Term<Exponent, BaseRing>>

// inspired by the matrix macro
macro_rules! gens {
    () => {
        //
    };
}

// pub trait DynRing {
// }

pub enum Expr<const N: usize, R: Ring2<N>> {
    Leaf(R::Elem),
    Add(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),
    // Sub(Self, Self),
    Mul(Box<Self>, Box<Self>),
    Div(Box<Self>, Box<Self>),
    Power(Box<Self>, usize),
}

pub trait Ring2<const N: usize>: Sized {
    type Elem: RingElem<N, Self>;

    // returns a ring, potentially itself if not quotient
    // fn base_ring(&self) ->
    // fn ideal
    // fn gens
    // fn coeffs()
}

pub trait NCRing<const N: usize> {
    //
    type Elem;
}

// D = degree
// N = ring gens
pub struct PolyRing<
    const D: usize,
    const N: usize,
    R: Ring2<N>,
> {
    ring: R,
    // coeffs: []
}

// A ring element i guess has a position

// from S -> R
pub struct Hom2<
    const A: usize,
    const B: usize,
    S: Ring2<A>,
    R: Ring2<B>,
> {
    domain: S,
    codomain: R,
    //
    images: [Expr<B, R>; B],
    ph: std::marker::PhantomData<(S, R)>,
}

impl<
        const A: usize,
        const B: usize,
        S: Ring2<A>,
        R: Ring2<B>,
    > Hom2<A, B, S, R>
{
    pub fn new(
        s: S,
        r: R,
        images: [Expr<B, R>; B],
    ) -> Self {
        Self {
            domain: s,
            codomain: r,
            images,
            ph: <_>::default(),
        }
    }

    fn get(&self, el: R::Elem) -> S::Elem {
        unimplemented!()
    }
}

// maybe hom should be parametrized by the
fn hom2<
    const A: usize,
    const B: usize,
    S: Ring2<A>,
    R: Ring2<B>,
>(
    // a: [usize; A],
    // b: [usize; B],
    // images: [usize; B],
    s: S,
    r: R,
    // note that th
    // images: [S::Elem; B],
    images: [Expr<B, R>; B],
) -> Hom2<A, B, S, R> {
    //
    Hom2::new(s, r, images)
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
        // self.a.si
        unimplemented!()
    }
}

pub fn zip_longest<A: Iterator, B: Iterator>(
    a: A,
    b: B,
) -> ZipLongest<A, B> {
    ZipLongest::new(a, b)
}

// pub trait IteratorExt {
// fn zip_longest(&self, other: )
// }
