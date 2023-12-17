// from macaulay2 and  sage

use crate::prelude::*;

pub struct HyperplaneArrangement {
    //
    data: Vec<Hyperplane>,
}


impl From<Matroid> for HyperplaneArrangement{
    fn from(value: Matroid) -> Self {
        unimplemented!()
    }
}