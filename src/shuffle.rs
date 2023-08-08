// #!/usr/bin/env python3

// import itertools

// def concatenate(u, words):
//     """Concatenates a letter with each word in an iterable."""

//     for word in words:
//         yield tuple([u] + list(word))

// def shuffle(w1, w2):
//     """Computes the shuffle product of two words."""

//     if len(w1) == 0:
//         return [w2]

//     if len(w2) == 0:
//         return [w1]

//     gen1 = concatenate(w1[0], shuffle(w1[1:], w2))
//     gen2 = concatenate(w2[0], shuffle(w1, w2[1:]))

//     return itertools.chain(gen1, gen2)

// print(list(shuffle([1,2,3], [1,2,3])))
///
/// https://github.com/imanolperez/optimal-double-execution/blob/b380087765925043b01fe2f1066e5e2d1d850cf9/src/shuffle.py
///
fn concat(e: usize, i: &[usize]) -> impl Iterator<Item=usize> + '_ {
    // std::iter::from_fn(|| None)
    std::iter::once(e).chain(i.iter().cloned())
}

pub fn shuffle(a: &[usize], b: &[usize]) {
    //
    // concat()
    if a.is_empty() {
        //
    }

    if b.is_empty() {
        //
    }

    // concat(a[0], shuffle(&a[1..], w2))

}

mod test {
    #[test]
    fn test_shuffle() {
        //
    }
}