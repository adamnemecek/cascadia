pub enum Expr1 {
    Leaf(usize),
    Add(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
    Div(Box<Self>, Box<Self>),
    Power(Box<Self>, Box<Self>),
}

// impl<const N: usize, R: Ring2<N>> std::ops::Add
//     for Expr<N, R>
// {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self {
//         Self::Add(self.into(), rhs.into())
//     }
// }

// impl<const N: usize, R: Ring2<N>> std::ops::Sub
//     for Expr<N, R>
// {
//     type Output = Self;
//     fn sub(self, rhs: Self) -> Self {
//         Self::Sub(self.into(), rhs.into())
//     }
// }

// (- $(t:tt)*) => { ... };
macro_rules! expr {
    ($left:tt ^ $right:tt) => {
        Expr1::Power(
            expr!($left).into(),
            expr!($right).into(),
        )
    };
    ($left:tt * $right:tt) => {
        //
    };
    ($left:tt / $right:tt) => {
        //
    };
    ($left:tt + $right:tt) => {
        //
    };
    ($left:tt - $right:tt) => {
        //
    };

    ($v: tt) => {
        Expr1::Leaf($v)
    };
}

fn test1() {
    expr!(10 ^ 10);
}
