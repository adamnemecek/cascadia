use crate::prelude::*;

pub struct UnivariatePolynomial<S: Ring> {
    inner: Vec<S>, //
}

//

impl<S: Ring> std::ops::Add for UnivariatePolynomial<S> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
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

impl<S: Ring> std::ops::Sub for UnivariatePolynomial<S> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        unimplemented!()
    }
}

impl<S: Ring> std::ops::Mul for UnivariatePolynomial<S> {
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

// impl<S: Ring> Ring for UnivariatePolynomial<S> {
//     //
// }

mod tests {
    #[test]
    fn test_new() {
        //
        let p = poly!(ZZ, x);
    }
}
