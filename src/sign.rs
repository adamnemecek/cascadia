#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum Sign {
    Pos,
    Zero,
    Neg,
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
