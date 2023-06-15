#[derive(Debug)]
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

macro_rules! binop {
    ($op: tt, $left: tt, $right: tt) => {
        Expr1::$op(
            expr!($left).into(),
            expr!($right).into(),
        )
    };
}

// (- $(t:tt)*) => { ... };
macro_rules! expr {
    ($left:tt ^ $right:tt) => {
        binop!(Power, $left, $right)
        // Expr1::Power(
        //     expr!($left).into(),
        //     expr!($right).into(),
        // )
    };
    ($left:tt * $right:tt) => {
        Expr1::Mul(
            expr!($left).into(),
            expr!($right).into(),
        )
    };
    ($left:tt / $right:tt) => {
        Expr1::Div(
            expr!($left).into(),
            expr!($right).into(),
        )
    };
    ($left:tt + $right:tt) => {
        Expr1::Add(
            expr!($left).into(),
            expr!($right).into(),
        )
    };
    ($left:tt - $right:tt) => {
        Expr1::Sub(
            expr!($left).into(),
            expr!($right).into(),
        )
    };

    ($v: tt) => {
        Expr1::Leaf($v)
    };
}

fn test1() {
    //
}

mod tests {
    use super::Expr1;

    #[test]
    fn test_expr_parser() {
        let a = expr!(10 ^ 10);
        println!("{:?}", a);
    }
}
