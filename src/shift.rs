fn f(x: f64) -> f64 {
    // 2.0 * x + 4.0
    x + 2.0 * x + x.powf(4.0) + 5.0
}

pub fn op<T: Copy>(
    f: impl Fn(T) -> T + Clone,
    t: T,
    op: impl Fn(T, T) -> T,
) -> impl Fn(T) -> T {
    move |x| f.clone()(op(x, t))
}

// shift operator is uncurring
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

// fn exp_shift(t: f64, ddx: bool) {
//     //
// }

fn is_approx(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.01
}

// pub trait Shift<T> {
//     fn shift(&self, t: T) -> Self;
// }

// impl<T, F: Fn(T) -> T + Clone> Shift<T> for Box<F> {
//     fn shift(&self, t: T) -> Box<F> {
//         // move |x| (self.clone())(x + t)
//         Box::new(move |x| self.clone()(x + t))
//     }
// }

fn div<A: Copy, B: std::ops::Div<Output = B>>(
    a: impl Fn(A) -> B,
    b: impl Fn(A) -> B,
) -> impl Fn(A) -> B {
    move |x| a(x) / b(x)
    //
}

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

///
///
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

// fn test_shift() {
//     // exp(t * d/dx) * f(x) = f(x + t)
//     // shift(f, 3.0);
//     println!("{}", test_shift_op(f, 3.0, 5.0));
// }

mod tests {
    use super::*;
    #[test]
    fn test_shift() {
        println!("{}", test_shift_op(f, 3.0, 3.0));
        println!(
            "{}",
            find_t(f, 3.0, test_shift_op(f, 3.0, 3.0))
        );
    }
}
