// from macaulay2 and  sage

use crate::prelude::*;

pub struct HyperplaneArrangement {
    //
    data: Vec<Hyperplane>,
}
impl HyperplaneArrangement {
    pub fn regions(&mut self) {
        //
    }

    pub fn vertices(&mut self) {
        //
    }

    pub fn insert(&mut self, x: Hyperplane) {
        unimplemented!()
    }

    pub fn matroid(&self) -> Matroid {
        unimplemented!()
    }
}

impl From<Matroid> for HyperplaneArrangement {
    fn from(value: Matroid) -> Self {
        unimplemented!()
    }
}
