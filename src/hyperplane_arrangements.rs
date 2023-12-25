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

    pub fn matroid(&self) -> LinearMatroid {
        unimplemented!()
    }
}

impl From<LinearMatroid> for HyperplaneArrangement {
    fn from(value: LinearMatroid) -> Self {
        unimplemented!()
    }
}
