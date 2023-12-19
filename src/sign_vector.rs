use crate::prelude::*;

// https://github.com/MarcusAichmayr/elementary_vectors

pub struct SignVec(Vec<Sign>);

pub trait IterExt<T> {
    fn positions(
        self,
        p: impl Fn(T) -> bool,
    ) -> impl Iterator<Item = usize>;
}

impl<T, I: Iterator<Item = T>> IterExt<T> for I {
    fn positions(
        self,
        p: impl Fn(T) -> bool,
    ) -> impl Iterator<Item = usize> {
        self.enumerate().filter_map(move |(i, e)| {
            if p(e) {
                i.into()
            } else {
                None
            }
        })
    }
}

impl SignVec {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn iter(&self) -> impl Iterator<Item = &Sign> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn zero_support(&self) -> Vec<usize> {
        self.iter().positions(|x| x.is_zero()).collect()
    }
}

impl std::ops::Index<usize> for SignVec {
    type Output = Sign;

    fn index(&self, index: usize) -> &Sign {
        &self.0[index]
    }
}
