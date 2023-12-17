use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    Front,
    Back,
}

impl Dir {
    pub fn is_front(&self) -> bool {
        self == &Self::Front
    }

    pub fn flip(&self) -> Self {
        if self.is_front() {
            Self::Back
        } else {
            Self::Front
        }
    }
}

// struct Flip {

// }

// shift operator
// what if i keep track of the signs
struct OrientedFeedback<T> {
    dir: Dir,
    q: VecDeque<T>,
}

// this feels like some sort of reflection and

impl<T> OrientedFeedback<T> {
    pub fn new() -> Self {
        Self {
            dir: Dir::Front,
            q: <_>::default(),
        }
    }

    pub fn flip(&mut self, dir: Dir) {
        self.dir = dir;
        // what if i keep track of this and what if i keep track of how many iterations i did
    }

    pub fn push(&mut self, e: T) {
        if self.dir.is_front() {
            self.q.push_front(e);
        } else {
            self.q.push_back(e);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.q.iter()
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.dir.is_front() {
            self.q.pop_front()
        } else {
            self.q.pop_back()
        }
    }
}

// pub enum Either<A, B> {
//     Left(A),
//     Right(B),
// }

// use std::collections::VecDeque;

// struct Feedback<A, B> {
//     q: VecDeque<Either<A, B>>,
// }

// impl<A, B> Feedback<A, B> {
//     pub fn new() -> Self {
//         unimplemented!()
//     }
// }

// // pub struct Cogwheels<T> {
// //     consumer: VecDeque<T>,
// //     producer: VecDeque<T>,
// // }

// // impl<T> Cogwheels<T> {
// //     //
// // }

mod tests {
    use std::collections::VecDeque;

    #[test]
    fn test_feedback() {
        let mut q = VecDeque::<usize>::new();
        let mut r = VecDeque::<usize>::new();

        q.extend(0..10);
        for e in q {
            println!("{:?}", e);
        }
        // let mut q = VecDeque::new();
        // q.extend(0..10);

        // while let Some(e) = q.pop_front() {
        //     //
        //     if e < 10 {
        //         q.extend(e..e + 10);
        //         println!("extend {}", e);
        //     } else {
        //         println!("pop {}", e);
        //     }

        // it's as if you were
        // }
    }
}
