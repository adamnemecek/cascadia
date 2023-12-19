#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum Sign {
    Neg,
    Zero,
    Pos,
}

impl Sign {
    pub fn new(v: isize) -> Self {
        if v == 0 {
            Self::Zero
        } else if v.is_positive() {
            Self::Pos
        } else {
            Self::Neg
        }
    }

    pub fn raw(&self) -> isize {
        match self {
            Self::Neg => -1,
            Self::Zero => 0,
            Self::Pos => 1,
        }
    }
}

impl std::fmt::Debug for Sign {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Neg => write!(f, "-"),
            Self::Zero => write!(f, "0"),
            Self::Pos => write!(f, "+"),
        }
    }
}

impl std::fmt::Display for Sign {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::ops::Neg for Sign {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Self::Pos => Self::Neg,
            Self::Zero => Self::Zero,
            Self::Neg => Self::Pos,
        }
    }
}
