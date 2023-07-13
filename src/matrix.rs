// https://github.com/debasish-raychawdhuri/linearalgebra-rust/blob/master/src/lib.rs

use std::ops::*;

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

pub trait Field: RingOps {
    fn inv(
        &self,
        value: &Self::Element,
    ) -> Result<Self::Element, Error>;
}

#[macro_export]
macro_rules! matrix {
    ($RingOps:expr,[$($($ex:expr),*);*]) => {
        {
            let data = [$(
                [$(
                    $ex,
                )*],

            )*];
            Matrix::new_from_array($RingOps, data)
        }
    };
}

#[derive(PartialEq, Clone, Debug)]
pub struct Matrix<'a, F: RingOps> {
    RingOps: &'a F,
    rows: usize,
    columns: usize,
    data: Vec<Vec<F::Element>>,
}

impl<'a, F: RingOps> Matrix<'a, F> {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn value_at(
        &self,
        row: usize,
        col: usize,
    ) -> F::Element {
        self.data[row][col].clone()
    }
}

impl<'a, F: Field> Matrix<'a, F> {
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
            data1[r1][i] = self.RingOps.add(
                &data1[r1][i],
                &self.RingOps.mul(&mul, &data1[r2][i]),
            );
            data2[r1][i] = self.RingOps.add(
                &data2[r1][i],
                &self.RingOps.mul(&mul, &data2[r2][i]),
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
                self.RingOps.mul(&mul, &data1[r1][i]);
            data2[r1][i] =
                self.RingOps.mul(&mul, &data2[r1][i]);
        }
    }

    fn find_non_zero_pivot(
        &self,
        data1: &Vec<Vec<F::Element>>,
        start: usize,
    ) -> Result<usize, Error> {
        for i in start..self.rows {
            if data1[i][start] != self.RingOps.zero() {
                return Ok(i);
            }
        }
        Err(Error::InversionOfNonInvertibleSquareMatrix)
    }

    pub fn inverse(&self) -> Result<Self, Error> {
        if self.rows != self.columns {
            return Err(
                Error::InversionOfRectangularMatrix,
            );
        }
        let mut data1 = self.data.clone();
        let mut data2 =
            Self::one(self.RingOps, self.rows).data;

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
            match self.RingOps.inv(&d) {
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
                let mult = self.RingOps.neg(&data1[j][i]);
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
                    self.RingOps.neg(&data1[y][x].clone());
                self.add_multiple_of(
                    &mut data1, &mut data2, y, x, mult,
                );
            }
        }

        return Ok(Matrix::new(self.RingOps, data2));
    }
}

impl<'a, F: RingOps> Matrix<'a, F> {
    pub fn new(
        RingOps: &'a F,
        v: Vec<Vec<F::Element>>,
    ) -> Self {
        let rows = v.len();
        let columns = v[0].len();
        let data = v;
        Self {
            RingOps,
            rows,
            columns,
            data,
        }
    }

    pub fn new_from_array<
        const ROWS: usize,
        const COLS: usize,
    >(
        RingOps: &'a F,
        data: [[F::Element; COLS]; ROWS],
    ) -> Self {
        let mut v = Vec::new();
        for i in 0..ROWS {
            let row = data[i].to_vec();
            v.push(row);
        }
        Self {
            RingOps,
            rows: ROWS,
            columns: COLS,
            data: v,
        }
    }

    pub fn one(RingOps: &'a F, rows: usize) -> Self {
        let mut data =
            vec![vec![RingOps.zero(); rows]; rows];
        for i in 0..rows {
            data[i][i] = RingOps.one();
        }
        Self {
            RingOps,
            rows,
            columns: rows,
            data,
        }
    }

    pub fn zero(RingOps: &'a F, rows: usize) -> Self {
        let data = vec![vec![RingOps.zero(); rows]; rows];
        Self {
            RingOps,
            rows,
            columns: rows,
            data,
        }
    }

    pub fn scale(&self, scalar: F::Element) -> Matrix<F> {
        let mut ans: Matrix<F> = Matrix {
            RingOps: self.RingOps,
            rows: self.rows,
            columns: self.columns,
            data: vec![
                vec![
                    self.RingOps.zero();
                    self.columns
                ];
                self.rows
            ],
        };
        for i in 0..self.rows {
            for j in 0..self.columns {
                ans.data[i][j] = self
                    .RingOps
                    .mul(&self.data[i][j], &scalar);
            }
        }
        ans
    }

    pub fn add(
        &self,
        rhs: &Matrix<'a, F>,
    ) -> Result<Matrix<'a, F>, Error> {
        if self.rows != rhs.rows
            || self.columns != rhs.columns
        {
            Result::Err(
                Error::DimensionMismatchForMatrixAddition(
                    self.rows,
                    self.columns,
                    rhs.rows,
                    rhs.columns,
                ),
            )
        } else {
            let mut ans: Matrix<F> = Matrix {
                RingOps: self.RingOps,
                rows: self.rows,
                columns: self.columns,
                data: vec![
                    vec![
                        self.RingOps.zero();
                        self.columns
                    ];
                    self.rows
                ],
            };
            for i in 0..self.rows {
                for j in 0..self.columns {
                    ans.data[i][j] = self.RingOps.add(
                        &self.data[i][j],
                        &rhs.data[i][j],
                    );
                }
            }
            Ok(ans)
        }
    }

    pub fn sub(
        &self,
        rhs: &Matrix<F>,
    ) -> Result<Matrix<F>, Error> {
        if self.rows != rhs.rows
            || self.columns != rhs.columns
        {
            Result::Err(
                Error::DimensionMismatchForMatrixAddition(
                    self.rows,
                    self.columns,
                    rhs.rows,
                    rhs.columns,
                ),
            )
        } else {
            let mut ans: Matrix<F> = Matrix {
                RingOps: self.RingOps,
                rows: self.rows,
                columns: self.columns,
                data: vec![
                    vec![
                        self.RingOps.zero();
                        self.columns
                    ];
                    self.rows
                ],
            };
            for i in 0..self.rows {
                for j in 0..self.columns {
                    ans.data[i][j] = self.RingOps.add(
                        &self.data[i][j],
                        &self.RingOps.neg(&rhs.data[i][j]),
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
    ) -> Result<Matrix<F>, Error> {
        if self.columns != rhs.rows {
            Result::Err(Error::DimensionMismatchForMatrixMultiplication(
                self.rows,
                self.columns,
                rhs.rows,
                rhs.columns,
            ))
        } else {
            let mut ans: Matrix<F> = Matrix {
                RingOps: self.RingOps,
                rows: self.rows,
                columns: rhs.columns,
                data: vec![
                    vec![
                        self.RingOps.zero();
                        rhs.columns
                    ];
                    self.rows
                ],
            };
            for i in 0..self.rows {
                for j in 0..rhs.columns {
                    for k in 0..self.columns {
                        let prod = self.RingOps.mul(
                            &self.data[i][k],
                            &rhs.data[k][j],
                        );
                        ans.data[i][j] = self
                            .RingOps
                            .add(&ans.data[i][j], &prod);
                    }
                }
            }
            Ok(ans)
        }
    }
    pub fn transpose(&self) -> Matrix<'a, F> {
        let rows = self.columns;
        let columns = self.rows;
        let mut ans: Matrix<F> = Matrix {
            RingOps: self.RingOps,
            rows,
            columns,
            data: vec![
                vec![self.RingOps.zero(); columns];
                rows
            ],
        };
        for j in 0..columns {
            for i in 0..rows {
                ans.data[i][j] = self.data[j][i].clone();
            }
        }
        ans
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

impl<'a, F: Field> Div<&Matrix<'a, F>>
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
