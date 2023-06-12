use crate::prelude::*;

pub struct UnivariatePolynomial<S: Ring> {
    inner: Vec<S>, //
}

//

impl<S: Ring> std::ops::Add for UnivariatePolynomial<S> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl<S: Ring> std::ops::Mul for UnivariatePolynomial<S> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
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
    fn test_new() {}
}
