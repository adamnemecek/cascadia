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

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Index<usize> for SignVec {
    type Output = Sign;

    fn index(&self, index: usize) -> &Sign {
        &self.0[index]
    }
}