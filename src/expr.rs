// https://stackoverflow.com/questions/36721733/is-there-a-way-to-pattern-match-infix-operations-with-precedence-in-rust-macros

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

// ($($name:ty,)*)
macro_rules! binop {
    ($op: tt, $left: tt, $right: tt) => {
        Expr1::$op(
            expr!($left).into(),
            expr!($right).into(),
        )
    };
}

macro_rules! expr {
    (($left:tt) ^ $right:tt) => {
        binop!(Power, $left, $right)
    };
    ($left:tt * $right:tt) => {
        binop!(Mul, $left, $right)
    };
    ($left:tt / $right:tt) => {
        binop!(Div, $left, $right)
    };
    ($left:tt + $right:tt) => {
        binop!(Add, $left, $right)
    };
    ($left:tt - $right:tt) => {
        binop!(Sub, $left, $right)
    };
    ($v: tt) => {
        Expr1::Leaf($v)
    };
}

#[derive(Debug)]
pub enum Expr {
    Var(String),
    Lit(f64),
    Exp(Box<Self>),
    Sin(Box<Self>),
    Cos(Box<Self>),
    Add(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
    Div(Box<Self>, Box<Self>),
}

// Macro DSL for exprs
macro_rules! expr {
    (@expt $fun:ident($($t:tt)*)) => {Expr::$fun(Box::new(expr!($($t)*)))};
    (@expt $col:ident) => {Expr::Var(stringify!($col).to_string())};
    (@expt $val:literal) => {Expr::Lit($val)};
    (@expt ($($t:tt)*)) => {(expr!($($t)*))};
    // Look for /
    (@exp/ [$($x:tt)*]) => {expr!(@expt $($x)*)}; // We are done, look for lower priority ops
    (@exp/ [$($x:tt)*] / $($t:tt)*) => {Expr::Div(Box::new(expr!(@expt $($x)*)), Box::new(expr!(@exp/ $($t)*)))}; // Consume until the op
    (@exp/ [$($x:tt)*] $h:tt $($t:tt)*) => {expr!(@exp/ [$($x)* $h] $($t)*)}; // Consume the tokens until we find the right op
    (@exp/ $($t:tt)*) => {expr!(@exp/ [] $($t)*)}; // Start consuming tokens
    // Look for *
    (@exp* [$($x:tt)*]) => {expr!(@exp/ $($x)*)}; // We are done, look for lower priority ops
    (@exp* [$($x:tt)*] * $($t:tt)*) => {Expr::Mul(Box::new(expr!(@exp/ $($x)*)), Box::new(expr!(@exp* $($t)*)))}; // Consume until the op
    (@exp* [$($x:tt)*] $h:tt $($t:tt)*) => {expr!(@exp* [$($x)* $h] $($t)*)}; // Consume the tokens until we find the right op
    (@exp* $($t:tt)*) => {expr!(@exp* [] $($t)*)}; // Start consuming tokens
    // Look for -
    (@exp- [$($x:tt)*]) => {expr!(@exp* $($x)*)}; // We are done, look for lower priority ops
    (@exp- [$($x:tt)*] - $($t:tt)*) => {Expr::Sub(Box::new(expr!(@exp* $($x)*)), Box::new(expr!(@exp- $($t)*)))}; // Consume until the op
    (@exp- [$($x:tt)*] $h:tt $($t:tt)*) => {expr!(@exp- [$($x)* $h] $($t)*)}; // Consume the tokens until we find the right op
    (@exp- $($t:tt)*) => {expr!(@exp- [] $($t)*)}; // Start consuming tokens
    // Look for +
    (@exp+ [$($x:tt)*]) => {expr!(@exp- $($x)*)}; // We are done, look for lower priority ops
    (@exp+ [$($x:tt)*] + $($t:tt)*) => {Expr::Add(Box::new(expr!(@exp- $($x)*)), Box::new(expr!(@exp+ $($t)*)))}; // Consume until the op
    (@exp+ [$($x:tt)*] $h:tt $($t:tt)*) => {expr!(@exp+ [$($x)* $h] $($t)*)}; // Consume the tokens until we find the right op
    (@exp+ $($t:tt)*) => {expr!(@exp+ [] $($t)*)}; // Start consuming tokens
    // Look for high priority ops first
    ($($t:tt)*) => {expr!(@exp+ $($t)*)};
}

fn test1() {
    //
    let ex = expr!(Exp(a * b + Cos(2. * z) * d
        - 2. * (y + 3.)
        + t * Sin(c + 3. * t)));
}

mod tests {
    use super::Expr1;

    #[test]
    fn test_expr_parser() {
        // let a = expr!(10 ^ 10 + 10 / 10);
        // let a = expr!((10 + 10) ^ 10);
        // println!("{:?}", a);
    }
}
