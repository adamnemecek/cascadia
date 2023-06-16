// pub trait FreeModule<const N

pub trait NCRing3Elem<const N: usize, R: NCRing3<N, Self>>:
    Sized
{
    //
}
pub trait NCRing3<const N: usize, E: NCRing3Elem<N, Self>>:
    Sized
{
    // fn base_ring(&self) ->
}

pub trait Ring3Elem<const N: usize, R: Ring3<N, Self>>:
    NCRing3Elem<N, R>
{
}
pub trait Ring3<const N: usize, E: Ring3Elem<N, Self>>:
    NCRing3<N, E>
{
}

// fn a<const N: usize, R: Ring3<N, E>, E: Ring3Elem<N, R>>(
//     a: R,
// ) {
//     //
// }

// LinearCombination
pub struct Term {
    base: String,
    coeff: usize,
    power: usize,
}

impl std::ops::Add for Term {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        // Self
        unimplemented!()
    }
}

impl Term {
    // cz// pub fn new(base: String, )
}

pub struct LinComb {}
