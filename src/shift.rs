fn f(x: f64) -> f64 {
    // 2.0 * x + 4.0
    x + 2.0 * x + x.powf(4.0) + 5.0
}

///
/// wraps the function in a way
///
pub fn op<T: Copy>(
    f: impl Fn(T) -> T + Clone,
    t: T,
    op: impl Fn(T, T) -> T,
) -> impl Fn(T) -> T {
    move |x| f.clone()(op(x, t))
}

///
/// shift operator is uncurring
///
///
pub fn left_shift<T: Copy + std::ops::Add<Output = T>>(
    f: impl Fn(T) -> T + Clone,
    t: T,
) -> impl Fn(T) -> T {
    op(f, t, T::add)
}

pub fn right_shift<T: Copy + std::ops::Sub<Output = T>>(
    f: impl Fn(T) -> T + Clone,
    t: T,
) -> impl Fn(T) -> T {
    op(f, t, T::sub)
}

// pub fn taylor<T>(f: impl Fn(T) -> T) {
//
// }

// fn exp_shift(t: f64, ddx: bool) {
//     //
// }

fn is_approx(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.01
}

pub trait Shift<T> {
    type Output;
    fn shift(&self, t: T) -> Self::Output;
}

// impl<
//         T: Copy + std::ops::Add<Output = T>,
//         F: Fn(T) -> T + Clone,
//     > Shift<T> for F
// {
//     type Output = impl Fn(T) -> T;
//     fn shift(&self, t: T) -> Self::Output {
//         left_shift(self.clone(), t)
//     }
// }

// impl<T, I: Iterator<Item = T>> Shift<T> for I {
//     type Output = Self;
//     fn shift(&self, t: T) -> Self::Output {
//         // left_shift(self.clone(), t)
//         unimplemented!()
//     }
// }

// we know t and want the action
fn test_shift_op(
    f: impl Clone + Fn(f64) -> f64,
    x: f64,
    t: f64,
) -> f64 {
    let s = left_shift(f.clone(), t);

    let fx = f(x);

    let sx = s(x);
    // println!("{} {}", fx, rhs);
    let ddx = (sx / fx).ln() / t;
    let action = (t * ddx).exp();

    let lhs = fx * action;
    // println!("result {lhs} {rhs}");
    assert!(is_approx(lhs, sx));
    action
}

fn fn_op<A: Copy, B>(
    a: impl Fn(A) -> B,
    b: impl Fn(A) -> B,
    op: impl Fn(B, B) -> B,
) -> impl Fn(A) -> B {
    move |x| op(a(x), b(x))
}

fn div<A: Copy, B: std::ops::Div<Output = B>>(
    a: impl Fn(A) -> B,
    b: impl Fn(A) -> B,
) -> impl Fn(A) -> B {
    fn_op(a, b, B::div)
}

///
/// the original
///
pub fn shift_op(
    f: impl Clone + Fn(f64) -> f64,
    x: f64,
    t: f64,
) -> f64 {
    let s = left_shift(f.clone(), t);
    s(x) / f(x)
}

// i kind of like this approach better
pub fn left_shift_op(
    f: impl Clone + Fn(f64) -> f64,
    t: f64,
) -> impl Fn(f64) -> f64 {
    div(left_shift(f.clone(), t), f)
}

///
/// given action, figured out by how much we have to shift
///
fn find_t(
    f: impl Clone + Fn(f64) -> f64,
    x: f64,
    action: f64,
) -> f64 {
    let fx = f(x);
    let sx = fx * action;
    // action.ln() / action
    // let sx = fx * action;
    // unimplemented!()
    // t * d/dx = log(f(x + t) / f(x))
    let t = (sx / fx).ln() / action;
    t
}

///t
///trait
///
fn find_x(
    f: impl Clone + Fn(f64) -> f64,
    x: f64,
    action: f64,
) -> f64 {
    let fx = f(x);
    // let sx = fx * action;
    unimplemented!()
}

fn find_hyperplane(f: impl Fn(f64) -> f64) {
    //
}

// fn binomial_series(alpha: f64, n: usize) -> impl Iterator<Item = f64> {
//     // https://gist.github.com/powerwlsl/dd3d5e199645ee1f49701ea996893aec

//     // tmp = a^n
//     // answer = tmp
//     // for (i = 1; i <= n; i += 1) {
//     // tmp =  (tmp * (n-i+1) * x )/(a * i)
//     // answer = answer + tmp
//     // }

//     let mut tmp = alpha.powf(n as _);
//     let mut answer = tmp;
//     let mut it = 0..n;

//     std::iter::from_fn(|| {
//         let Some(n) = it.next() else { return None };
//         tmp = (tmp * (n - i + 1));
//         None
//     })
// }

fn binomial(n: usize, mut k: usize) -> usize {
    // Since binomial(n, k) = binomial(n, n - k), we might as well use
    // the smaller k to optimize
    if n - k < k {
        k = n - k;
    }

    // Compute the coefficient
    let mut res: usize = 1;
    for i in 1..k + 1 {
        // res = res * FromPrimitive::from_uint(n - k + i).unwrap();
        // res = res / FromPrimitive::from_uint(i).unwrap();
    }

    res
}

// fn test_shift() {
//     // exp(t * d/dx) * f(x) = f(x + t)
//     // shift(f, 3.0);
//     println!("{}", test_shift_op(f, 3.0, 5.0));
// }

mod tests {
    use super::*;
    #[test]
    fn test_shift() {
        // let z = f.shift(3.0)(3.0);
        // let z: f64 = f.shift(3.0)(3.0); // does not work
        // println!("{}", z);
        // let z = f.shift(-3.0);
        println!("{}", left_shift_op(f, 3.0)(4.0));
        println!("{}", test_shift_op(f, 3.0, 3.0));
        println!(
            "{}",
            find_t(f, 3.0, test_shift_op(f, 3.0, 3.0))
        );
    }
}

// left_shift(t, f::Function) = x -> f(x+t)/f(x)
// tay()
