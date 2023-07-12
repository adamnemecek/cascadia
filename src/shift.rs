fn f(x: f64) -> f64 {
    x + 2.0
}

fn shift<T: Copy + std::ops::Add<Output = T>>(
    f: impl Fn(T) -> T + Clone,
    t: T,
) -> impl Fn(T) -> T {
    move |x| f.clone()(x + t)
}

fn exp_shift(t: f64, ddx: bool) {
    //
}

fn is_approx(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.01
}

pub trait Shift<T> {
    fn shift(&self, t: T) -> Self;
}

// impl<T, F: Fn(T) -> T + Clone> Shift<T> for Box<F> {
//     fn shift(&self, t: T) -> Box<F> {
//         // move |x| (self.clone())(x + t)
//         Box::new(move |x| self.clone()(x + t))
//     }
// }

fn test_shift1(
    f: impl Clone + Fn(f64) -> f64,
    x: f64,
    t: f64,
) -> bool {
    let s = shift(f.clone(), t);

    let fx = f(x);

    let rhs = s(x);
    println!("{} {}", fx, rhs);
    let ddx = (rhs / fx).ln() / t;
    let lhs = (t * ddx).exp();
    println!("result {lhs} {rhs}");
    lhs == rhs
}

fn test_shift() {
    // exp(t * d/dx) * f(x) = f(x + t)
    // shift(f, 3.0);
    println!("{}", test_shift1(f, 2.0, 4.0));
}

mod tests {
    use super::*;
    #[test]
    fn test_shift() {
        println!("{}", test_shift1(f, 2.0, 4.0));
    }
}
