// k vector space over b
// based on h4m
#[derive(Clone)]
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
        // self.0.group_by_
        // unimplemented!()
    }
}


mod tests {
    #[test]
    fn test1() {
        //
    }
}