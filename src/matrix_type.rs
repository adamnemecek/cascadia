pub trait MatrixType<T>:
    std::ops::Index<(usize, usize), Output = T>
{
    fn shape(&self) -> (usize, usize);

    fn diag(
        &self,
        index: impl Into<Option<isize>>,
    ) -> usize {
        0
    }

    fn antidiag(
        &self,
        index: impl Into<Option<isize>>,
    ) -> usize {
        0
    }
}

pub trait MatrixTypeMut<T>:
    MatrixType<T>
    + std::ops::IndexMut<(usize, usize), Output = T>
{
    //
}
