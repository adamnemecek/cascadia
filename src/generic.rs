pub trait DivRem<Rhs = Self> {
    type Output;
    fn div_rem(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_div_rem {
    ($ty: ty) => {
        impl DivRem for $ty {
            type Output = (Self, Self);
            fn div_rem(self, rhs: Self) -> Self::Output {
                (self / rhs, self % rhs)
            }
        }
    };
}

impl_div_rem!(usize);
