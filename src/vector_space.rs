// k vector space over b
// based on h4m
pub struct Vect<K, B>(Vec<(K, B)>);

impl<K, B> Vect<K, B> {
    fn coeff(&self, k: K) -> B {
        unimplemented!()
    }

    fn remove(&self) {
        //
    }

    fn nf(&mut self)
    where
        B: Ord,
    {
        self.0.sort_by(|(_, a), (_, b)| a.cmp(&b));
        unimplemented!()
    }
}
