use crate::prelude::*;

pub struct MPoly {
    //
    vars: Vec<String>,
    terms: Vec<Term>,
}

impl MPoly {
    //
    pub fn new() -> Self {
        Self {
            vars: vec![],
            terms: vec![],
        }
    }
}

impl std::ops::Add for MPoly {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        unimplemented!()
    }
}

impl std::ops::Mul for MPoly {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        unimplemented!()
    }
}
// lazard lifting/evaluation
// groeovner lifting
// is groebner for getting inverse?

// R = ZZ[x,y];
// S = ZZ[a,b,c];
// f = map(R,S,{x^2,x*y,y^2})

impl MPoly {
    // the point of this map is to allow you to evaluate the
    pub fn map(&self, other: &Self, images: &[MPoly]) {
        unimplemented!()
    }
}

mod test {
    #[test]
    fn test1() {
        //
        // let a = x + x^2 + y;
    }
}
