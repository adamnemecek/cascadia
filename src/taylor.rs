pub fn taylor(
    a: f64,
    f: impl Fn(f64) -> f64,
) -> impl Fn(f64) -> f64 {
    move |x| (x - a).exp() * f(x)
}

mod tests {
    use super::taylor;
    #[test]
    fn test_taylor() {
        let f = |x| x + 1.0;
        let q = taylor(0.0, f);

        println!("{}", q(0.0));
    }
}
