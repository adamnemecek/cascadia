pub struct Toeplitz<T> {
    //
    p: T,
}

pub trait SemiRing:
    Sized
    + std::ops::Add<Output = Self>
    + std::ops::Mul<Output = Self>
{
    //
}

impl SemiRing for f64 {}

pub struct DualM<const N: usize, T: SemiRing> {
    //
    p: T,
}
