use petgraph::matrix_graph::UnMatrix;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sign {
    Pos,
    Neg,
}

pub struct Complex<R> {
    pub re: R,
    pub im: R,
}

impl<R> Complex<R> {
    //
    pub fn arg(&self) {
        unimplemented!()
    }

    pub fn exp(&self) -> Self {
        unimplemented!()
    }

    pub fn sign(&self) -> Sign {
        // (Self::i * self.arg()).exp()
        unimplemented!()
    }
}

impl<R> std::ops::Mul for Complex<R> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        unimplemented!()
    }
}
