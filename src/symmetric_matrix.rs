pub struct SymmetricMatrix<T> {
    data: Vec<T>,
    n: usize,
}

impl<T> SymmetricMatrix<T> {
    //
    pub fn new(n: usize) -> Self {
        Self { n, data: vec![] }
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
        let (r, c) = index;
        let idx =
            if r < c {
                linear_index(self.n, r, c)
            } else {
                linear_index(self.n, c, r)
            };
        &self.data[idx]
    }
}

fn linear_index(n: usize, i: usize, j: usize) -> usize {
    i * (2 * n - i + 1) / 2 + j - i
}

mod tests {
    use super::linear_index;
    #[test]
    fn test_linear_index() {
        // [0 1 2 3
        //    4 5 6
        //      7 8
        //         9
        // ]
        // println!("linear index {}", linear_index(4, 0, 0));
        // println!("linear index {}", linear_index(4, 1, 1));
        println!("linear index {}", linear_index(4, 3, 3));
    }
}
