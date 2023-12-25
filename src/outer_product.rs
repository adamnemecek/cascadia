pub struct Outer<'a, A, B, T, F: Fn(&A, &B) -> T> {
    a: &'a [A],
    b: &'a [B],
    ph: std::marker::PhantomData<(A, B, T, F)>,
}
