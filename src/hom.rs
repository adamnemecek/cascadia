use crate::prelude::*;

// pub struct Hom3< {

// }

pub trait Map<Domain, Codomain> {
    //
}

impl<Domain, Codomain, F: Fn(Domain) -> Codomain>
    Map<Domain, Codomain> for F
{
    //
}

// R = ZZ[x,y];
// S = ZZ[a,b,c];
// f = map(R,S,{x^2,x*y,y^2})
// f(a)

//
// based on macaulay2
//
pub struct Hom<Domain: RingOps, Codomain: RingOps> {
    source: Domain,
    target: Codomain,
    // matrix: Matrix<
}
// https://github.com/oscar-system/Oscar.jl/blob/54ca9cb7dd42cbba6a451f77d86025fcaf6feeae/src/Rings/affine-algebra-homs.jl#L12
impl<Domain: RingOps, Codomain: RingOps>
    Hom<Domain, Codomain>
{
    //

    // from swiftymath
    pub fn then(&self) {
        //
    }

    pub fn kernel(&self) {
        unimplemented!()
    }

    pub fn image(&self) {
        unimplemented!()
    }
}
