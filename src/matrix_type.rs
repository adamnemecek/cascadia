pub trait MatrixType<T>:
    std::ops::Index<(usize, usize), Output = T>
{
    fn shape(&self) -> (usize, usize);

    fn diag(
        &self,
        index: impl Into<Option<isize>>,
    ) -> usize {
        // let index = index.into();
        // let from;
        // let to;
        let (from, to) = if let Some(index) = index.into() {
            if index < 0 {
                (0, 0)
            } else {
                (0, 0)
            }
            // unimplemented!();
        } else {
            (0, 0)
            // let from
            // unimplemented!();
        };
        0
    }

    fn antidiag(
        &self,
        index: impl Into<Option<isize>>,
    ) -> usize {
        0
    }
}

// pub struct

// pub trait D {

// }
// impl<T: D, M: MatrixType<T>> std::ops::Index<usize> for M {
//     type Output = T;
//     fn index(&self, index: usize) -> &Self::Output {
//         unimplemented!()
//     }
// }

pub trait MatrixTypeMut<T>:
    MatrixType<T>
    + std::ops::IndexMut<(usize, usize), Output = T>
{
    //
}
