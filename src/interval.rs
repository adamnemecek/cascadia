#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Interval<T>(T, T);

impl<T> Interval<T> {
    pub fn new(a: T, b: T) -> Self {
        Self(a, b)
    }
}
