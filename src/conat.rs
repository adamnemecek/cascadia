use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CoNat<T> {
    Finite(T),
    // infinity
    Omega,
}

// impl<T> CoNat<T> {
//     pub fn replace_with_infinity(&mut self) -> Option<T> {
//         if let Self::Finite(value) = std::mem::replace(self, Self::Omega) {
//             Some(value)
//         } else {
//             None
//         }
//     }
// }

// impl<T> CoNat<T> {
//     fn finite(&self) -> Option<&T> {

//     }
// }

impl<T, U> std::ops::Add<U> for CoNat<T>
where
    T: std::ops::Add<U>,
{
    type Output = CoNat<T::Output>;

    fn add(self, rhs: U) -> Self::Output {
        let Self::Finite(lhs) = self else {
            return CoNat::Omega;
        };
        CoNat::Finite(lhs + rhs)
    }
}

impl<T> PartialEq<T> for CoNat<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        let Self::Finite(lhs) = self else {
            return false;
        };
        lhs.eq(other)
    }
}

impl<T> PartialOrd<T> for CoNat<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        match self {
            Self::Finite(value) => value.partial_cmp(other),
            Self::Omega => Some(Ordering::Greater),
        }
    }
}

impl<T: PartialOrd> PartialOrd for CoNat<T> {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        // match self {
        //     Self::Finite(value) => value.partial_cmp(other),
        //     Self::Omega => Some(Ordering::Greater),
        // }
        match (self, other) {
            (Self::Finite(a), Self::Finite(b)) => {
                a.partial_cmp(b)
            }
            (Self::Omega, Self::Finite(_)) => {
                Ordering::Greater.into()
            }
            (Self::Finite(_), Self::Omega) => {
                Ordering::Less.into()
            }
            (Self::Omega, Self::Omega) => {
                Ordering::Equal.into()
            }
        }
    }
}

impl<T> From<T> for CoNat<T> {
    fn from(val: T) -> Self {
        Self::Finite(val)
    }
}

mod tests {
    use super::CoNat;
    #[test]
    fn test_conat() {
        //
    }
}

// pub struct KeyValuePair<K, V> {
//     pub key: K,
//     pub value: V,
// }

// impl<K, V> KeyValuePair<K, V> {
//     pub fn new(key: K, value: V) -> Self {
//         Self { key, value }
//     }
// }

// impl<K: PartialEq, V> PartialEq for KeyValuePair<K, V> {
//     fn eq(&self, other: &Self) -> bool {
//         self.key.eq(&other.key)
//     }
// }

// impl<K: Eq, V> Eq for KeyValuePair<K, V> {}

// impl<K: PartialOrd, V> PartialOrd for KeyValuePair<K, V> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         self.key.partial_cmp(&other.key)
//     }
// }

// impl<K: Ord, V> Ord for KeyValuePair<K, V> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.key.cmp(&other.key)
//     }
// }
