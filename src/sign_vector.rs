use crate::prelude::*;

// https://github.com/MarcusAichmayr/elementary_vectors

pub struct SignVec(Vec<Sign>);

impl SignVec {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn iter(&self) -> Iterator<Item=Sign> {
        self.0.iter()
    }
}

impl Index<usize> for SignVec {
    type Output = Sign;

    fn index(&self, index: usize) -> &Sign {
        unimplemented!()
    }
}