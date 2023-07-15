use crate::prelude::*;

pub struct Polynomial<S: Ring> {
    inner: Vec<S>, //
}

impl<S: Ring> Polynomial<S> {
    pub fn new(inner: Vec<S>) -> Self {
        Self { inner }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = S> + '_ + ExactSizeIterator
    {
        self.inner.iter().cloned()
    }

    pub fn convolve(
        &self,
        other: &Self,
        len: impl Into<Option<usize>>,
    ) -> Self {
        unimplemented!()
        // let len =
        //     len.into().unwrap_or(self.len() + other.len());
        // let mut inner = vec![S::zero(); len];

        // for (i, e) in self.iter().enumerate() {
        //     for (j, f) in other.iter().enumerate() {
        //         inner[(i + j) % len] += e.clone() * f;
        //     }
        // }

        // Self { inner }
    }
}

impl<S: Ring> FromIterator<S> for Polynomial<S> {
    fn from_iter<T: IntoIterator<Item = S>>(
        iter: T,
    ) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl<S: Ring> IntoIterator for Polynomial<S> {
    type Item = S;
    type IntoIter = <Vec<S> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

//

impl<S: Ring> std::ops::Add for Polynomial<S> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        // self.iter().zip_longest
        Self {
            inner: self
                .inner
                .into_iter()
                .zip(rhs.inner.into_iter())
                .map(|(a, b)| a + b)
                .collect(),
        }
    }
}

impl<S: Ring> std::ops::Sub for Polynomial<S> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        unimplemented!()
    }
}

impl<S: Ring> std::ops::Mul for Polynomial<S> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        unimplemented!()
    }
}

#[macro_export]
macro_rules! poly {
    () => {
        //
    };
}

// impl<S: Ring> Ring for Polynomial<S> {
//     //
// }

mod tests {
    #[test]
    fn test_new() {
        //
        // let p = poly!(ZZ, x);
    }
}
