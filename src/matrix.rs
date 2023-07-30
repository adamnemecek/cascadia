/// https://github.com/debasish-raychawdhuri/linearalgebra-rust/blob/master/src/lib.rs
use std::ops::*;

// pub trait AddOps<T> {
//     fn add(
//         &self,
//         lhs: &T,
//         rhs: &T,
//     ) -> T;
// }

pub trait RingOps {
    type Element: Clone + PartialEq;

    fn add(
        &self,
        lhs: &Self::Element,
        rhs: &Self::Element,
    ) -> Self::Element;

    fn mul(
        &self,
        lhs: &Self::Element,
        rhs: &Self::Element,
    ) -> Self::Element;

    fn neg(&self, lhs: &Self::Element) -> Self::Element;

    fn zero(&self) -> Self::Element;
    fn one(&self) -> Self::Element;
}

pub trait FieldOps: RingOps {
    fn inv(
        &self,
        value: &Self::Element,
    ) -> Result<Self::Element, Error>;
}

#[macro_export]
macro_rules! matrix {
    ($ring:expr,[$($($ex:expr),*);*]) => {
        {
            let data = [$(
                [$(
                    $ex,
                )*],

            )*];
            Matrix::new_from_array($ring, data)
        }
    };
}

#[derive(PartialEq, Clone, Debug)]
pub struct Matrix<'a, F: RingOps> {
    ring: &'a F,
    rows: usize,
    cols: usize,
    data: Vec<Vec<F::Element>>,
}

impl<'a, F: RingOps> Matrix<'a, F> {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    // pub fn value_at(
    //     &self,
    //     row: usize,
    //     col: usize,
    // ) -> F::Element {
    //     self.data[row][col].clone()
    // }

    pub fn cyclical_shift(n: usize, by: isize) -> Self {
        // diagind
        unimplemented!()
    }

    pub fn shift(n: usize, by: isize) -> Self {
        unimplemented!()
    }

    // like matlab
    // pub fn diag_ind(&self, )

    // pub fn get_diag(&self, d: isize) -> impl Iterator<Item=F> {
    //     unimplemented!()
    // }

    // pub fn set_diag(&self, d: isize) -> impl Iterator<Item=&mut F> {
    //     unimplemented!()
    // }
}

impl<'a, F: FieldOps> Matrix<'a, F> {
    fn swap_rows(
        &self,
        data1: &mut Vec<Vec<F::Element>>,
        data2: &mut Vec<Vec<F::Element>>,
        r1: usize,
        r2: usize,
    ) {
        if r1 == r2 {
            return;
        }
        data1.swap(r1, r2);
        data2.swap(r1, r2);
    }

    fn add_multiple_of(
        &self,
        data1: &mut Vec<Vec<F::Element>>,
        data2: &mut Vec<Vec<F::Element>>,
        r1: usize,
        r2: usize,
        mul: F::Element,
    ) {
        for i in 0..data1[r1].len() {
            data1[r1][i] = self.ring.add(
                &data1[r1][i],
                &self.ring.mul(&mul, &data1[r2][i]),
            );
            data2[r1][i] = self.ring.add(
                &data2[r1][i],
                &self.ring.mul(&mul, &data2[r2][i]),
            );
        }
    }

    fn scale_row(
        &self,
        data1: &mut Vec<Vec<F::Element>>,
        data2: &mut Vec<Vec<F::Element>>,
        r1: usize,
        mul: F::Element,
    ) {
        for i in 0..data1[r1].len() {
            data1[r1][i] =
                self.ring.mul(&mul, &data1[r1][i]);
            data2[r1][i] =
                self.ring.mul(&mul, &data2[r1][i]);
        }
    }

    fn find_non_zero_pivot(
        &self,
        data1: &Vec<Vec<F::Element>>,
        start: usize,
    ) -> Result<usize, Error> {
        let zero = self.ring.zero();
        for i in start..self.rows {
            if data1[i][start] != zero {
                return Ok(i);
            }
        }
        Err(Error::InversionOfNonInvertibleSquareMatrix)
    }

    pub fn inverse(&self) -> Result<Self, Error> {
        if self.rows != self.cols {
            return Err(
                Error::InversionOfRectangularMatrix,
            );
        }
        let mut data1 = self.data.clone();
        let mut data2 =
            Self::one(self.ring, self.rows).data;

        //triangulation of the matrix. make it an upper triangular matrix
        for i in 0..self.rows {
            let pivot = self.find_non_zero_pivot(&data1, i);
            match pivot {
                Ok(p) => {
                    self.swap_rows(
                        &mut data1, &mut data2, i, p,
                    );
                }
                Err(x) => {
                    return Err(x);
                }
            }

            let d = data1[i][i].clone();
            match self.ring.inv(&d) {
                Ok(d_inv) => {
                    self.scale_row(
                        &mut data1, &mut data2, i, d_inv,
                    );
                }
                Err(_) => {
                    return Err(Error::InversionOfNonInvertibleSquareMatrix);
                }
            }
            for j in i + 1..self.rows {
                let mult = self.ring.neg(&data1[j][i]);
                self.add_multiple_of(
                    &mut data1, &mut data2, j, i, mult,
                );
            }
        }

        //Now we make it a identity matrix. Notice that all diagonal entries are already 1

        for i in 0..self.rows {
            for j in i + 1..self.rows {
                let x = self.rows - i - 1;
                let y = self.rows - j - 1;
                let mult =
                    self.ring.neg(&data1[y][x].clone());
                self.add_multiple_of(
                    &mut data1, &mut data2, y, x, mult,
                );
            }
        }

        Ok(Self::new(self.ring, data2))
    }
}

impl<'a, F: RingOps> Matrix<'a, F> {
    pub fn new(
        ring: &'a F,
        v: Vec<Vec<F::Element>>,
    ) -> Self {
        let rows = v.len();
        let cols = v[0].len();
        let data = v;
        Self {
            ring,
            rows,
            cols,
            data,
        }
    }

    pub fn new_from_array<
        const ROWS: usize,
        const COLS: usize,
    >(
        ring: &'a F,
        data: [[F::Element; COLS]; ROWS],
    ) -> Self {
        let v =
            (0..ROWS).map(|i| data[i].to_vec()).collect();
        Self {
            ring,
            rows: ROWS,
            cols: COLS,
            data: v,
        }
    }

    pub fn diag(
        ring: &'a F,
        rows: usize,
        v: F::Element,
    ) -> Self {
        let mut data = vec![vec![ring.zero(); rows]; rows];
        for i in 0..rows {
            data[i][i] = v.clone();
        }
        Self {
            ring,
            rows,
            cols: rows,
            data,
        }
    }

    pub fn one(ring: &'a F, rows: usize) -> Self {
        Self::diag(ring, rows, ring.one())
    }

    pub fn zero(ring: &'a F, rows: usize) -> Self {
        let data = vec![vec![ring.zero(); rows]; rows];
        Self {
            ring,
            rows,
            cols: rows,
            data,
        }
    }

    pub fn scale(&self, scalar: F::Element) -> Self {
        let mut ans = Self {
            ring: self.ring,
            rows: self.rows,
            cols: self.cols,
            data: vec![
                vec![self.ring.zero(); self.cols];
                self.rows
            ],
        };
        for i in 0..self.rows {
            for j in 0..self.cols {
                ans[(i, j)] =
                    self.ring.mul(&self[(i, j)], &scalar);
            }
        }
        ans
    }

    pub fn dims(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn add(&self, rhs: &Self) -> Result<Self, Error> {
        if self.dims() != rhs.dims() {
            Result::Err(
                Error::DimensionMismatchForMatrixAddition(
                    self.rows, self.cols, rhs.rows,
                    rhs.cols,
                ),
            )
        } else {
            let mut ans = Self {
                ring: self.ring,
                rows: self.rows,
                cols: self.cols,
                data: vec![
                    vec![self.ring.zero(); self.cols];
                    self.rows
                ],
            };
            for i in 0..self.rows {
                for j in 0..self.cols {
                    ans[(i, j)] = self
                        .ring
                        .add(&self[(i, j)], &rhs[(i, j)]);
                }
            }
            Ok(ans)
        }
    }

    pub fn sub(&self, rhs: &Self) -> Result<Self, Error> {
        if self.dims() != rhs.dims() {
            Result::Err(
                Error::DimensionMismatchForMatrixAddition(
                    self.rows, self.cols, rhs.rows,
                    rhs.cols,
                ),
            )
        } else {
            let mut ans = Self {
                ring: self.ring,
                rows: self.rows,
                cols: self.cols,
                data: vec![
                    vec![self.ring.zero(); self.cols];
                    self.rows
                ],
            };
            for i in 0..self.rows {
                for j in 0..self.cols {
                    ans[(i, j)] = self.ring.add(
                        &self[(i, j)],
                        &self.ring.neg(&rhs[(i, j)]),
                    );
                }
            }
            Ok(ans)
        }
    }
    //vanila matrix multiplication
    pub fn mul(
        &self,
        rhs: &Matrix<F>,
    ) -> Result<Self, Error> {
        if self.cols != rhs.rows {
            Err(Error::DimensionMismatchForMatrixMultiplication(
                self.rows,
                self.cols,
                rhs.rows,
                rhs.cols,
            ))
        } else {
            let mut ans = Self {
                ring: self.ring,
                rows: self.rows,
                cols: rhs.cols,
                data: vec![
                    vec![self.ring.zero(); rhs.cols];
                    self.rows
                ],
            };
            for i in 0..self.rows {
                for j in 0..rhs.cols {
                    for k in 0..self.cols {
                        let prod = self.ring.mul(
                            &self[(i, k)],
                            &rhs[(k, j)],
                        );
                        ans[(i, j)] = self
                            .ring
                            .add(&ans[(i, j)], &prod);
                    }
                }
            }
            Ok(ans)
        }
    }

    pub fn transpose(&self) -> Self {
        let rows = self.cols;
        let cols = self.rows;
        let mut ans = Self {
            ring: self.ring,
            rows,
            cols,
            data: vec![vec![self.ring.zero(); cols]; rows],
        };
        for j in 0..cols {
            for i in 0..rows {
                ans[(i, j)] = self[(j, i)].clone();
            }
        }
        ans
    }
}

impl<'a, F: RingOps> std::ops::Index<(usize, usize)>
    for Matrix<'a, F>
{
    type Output = F::Element;
    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<'a, F: RingOps> std::ops::IndexMut<(usize, usize)>
    for Matrix<'a, F>
{
    fn index_mut(
        &mut self,
        index: (usize, usize),
    ) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

//Ops implemetations
impl<'a, F: RingOps> Add<&Matrix<'a, F>>
    for &'a Matrix<'a, F>
{
    type Output = Matrix<'a, F>;

    fn add(self, rhs: &Matrix<'a, F>) -> Matrix<'a, F> {
        match self.add(rhs) {
            Ok(result) => result,
            Err(e) => {
                panic!("{}", e.to_string());
            }
        }
    }
}

impl<'a, F: RingOps> Sub<&Matrix<'a, F>>
    for &'a Matrix<'a, F>
{
    type Output = Matrix<'a, F>;

    fn sub(self, rhs: &Matrix<'a, F>) -> Matrix<'a, F> {
        match self.sub(rhs) {
            Ok(result) => result,
            Err(e) => {
                panic!("{}", e.to_string());
            }
        }
    }
}

impl<'a, F: RingOps> Mul<&Matrix<'a, F>>
    for &'a Matrix<'a, F>
{
    type Output = Matrix<'a, F>;

    fn mul(self, rhs: &Matrix<'a, F>) -> Matrix<'a, F> {
        match self.mul(rhs) {
            Ok(result) => result,
            Err(e) => {
                panic!("{}", e.to_string());
            }
        }
    }
}

impl<'a, F: FieldOps> Div<&Matrix<'a, F>>
    for &'a Matrix<'a, F>
{
    type Output = Matrix<'a, F>;

    fn div(self, rhs: &Matrix<F>) -> Matrix<'a, F> {
        match rhs.inverse() {
            Ok(inv) => match self.mul(&inv) {
                Ok(result) => result,
                Err(e) => {
                    panic!("{}", e.to_string());
                }
            },
            Err(e) => {
                panic!("{}", e.to_string());
            }
        }
    }
}

use std::{
    error::Error as StdError,
    fmt::Display,
};

#[derive(Debug)]
pub enum Error {
    DivisionByZero,
    InversionOfZero,
    InversionOfNonInvertibleSquareMatrix,
    InversionOfRectangularMatrix,
    DimensionMismatchForMatrixAddition(
        usize,
        usize,
        usize,
        usize,
    ),
    DimensionMismatchForMatrixMultiplication(
        usize,
        usize,
        usize,
        usize,
    ),
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Error::DivisionByZero => write!(f, "Attempt to divide by zero"),
            Error::InversionOfZero => write!(f, "Attempt to invert zero"),
            Error::InversionOfNonInvertibleSquareMatrix => {
                write!(f, "Error trying to invert a non-invertible square matrix")
            }
            Error::InversionOfRectangularMatrix => {
                write!(f, "Error trying to invert a rectangular matrix")
            }
            Error::DimensionMismatchForMatrixAddition(rows1, cols1, rows2, cols2) => {
                write!(
                    f,
                    "Error trying to add two matrices of incompatible dimensions: \
                    ({}, {}) and ({}, {})",
                    rows1, cols1, rows2, cols2
                )
            }
            Error::DimensionMismatchForMatrixMultiplication(rows1, cols1, rows2, cols2) => {
                write!(
                    f,
                    "Error trying to multiply two matrices of incompatible dimensions: \
                    ({}, {}) and ({}, {})",
                    rows1, cols1, rows2, cols2
                )
            }
        }
    }
}

mod tests {
    #[test]
    fn test() {
        //
    }
}
