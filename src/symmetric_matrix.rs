pub struct SymmetricMatrix<T> {
    data: Vec<T>,
    n: usize,
}

impl<T: Default + Clone> SymmetricMatrix<T> {
    //
    pub fn new(n: usize) -> Self {
        let n = n * n / 2;
        Self {
            n,
            data: vec![<_>::default(); n],
        }
    }
}

impl<T> SymmetricMatrix<T> {
    fn linear_index(&self, index: (usize, usize)) -> usize {
        let (i, j) = index;
        i * (2 * self.n - i + 1) / 2 + j - i
    }
}

impl<T> std::ops::Index<(usize, usize)>
    for SymmetricMatrix<T>
{
    type Output = T;
    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output {
        // let (r, c) = index;

        &self.data[self.linear_index(index)]
    }
}

fn linear_index(n: usize, i: usize, j: usize) -> usize {
    i * (2 * n - i + 1) / 2 + j - i
}

mod tests {
    use super::SymmetricMatrix;
    #[test]
    fn test_linear_index() {
        // [0 1 2 3
        //    4 5 6
        //      7 8
        //         9
        // ]
        // println!("linear index {}", linear_index(4, 0, 0));
        // println!("linear index {}", linear_index(4, 1, 1));
        // println!("linear index {}", linear_index(4, 3, 3));
    }
}
