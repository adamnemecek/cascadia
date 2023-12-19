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

    pub fn is_zero(&self) -> bool {
        self == &Self::Zero
    }

    pub fn is_pos(&self) -> bool {
        self == &Self::Pos
    }

    pub fn is_neg(&self) -> bool {
        self == &Self::Neg
    }

    // this is essentialy like `.then` of Ordering
    pub fn compose(&self, other: &Self) -> Self {
        if !self.is_zero() {
            *self
        } else {
            *other
        }
    }
}

impl From<char> for Sign {
    fn from(value: char) -> Self {
        match value {
            '-' => Sign::Neg,
            '0' => Sign::Zero,
            '1' => Sign::Pos,
            _ => unimplemented!(),
        }
    }
}

impl From<isize> for Sign {
    fn from(value: isize) -> Self {
        if value == 0 {
            Self::Zero
        } else if value.is_negative() {
            Self::Pos
        } else {
            Self::Neg
        }
    }
}

impl From<Sign> for char {
    fn from(value: Sign) -> Self {
        match value {
            Sign::Neg => '-',
            Sign::Zero => '0',
            Sign::Pos => '1',
        }
    }
}

impl From<Sign> for isize {
    fn from(value: Sign) -> Self {
        match value {
            Sign::Neg => -1,
            Sign::Zero => 0,
            Sign::Pos => 1,
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

// #[test]
mod tests {
    #[test]
    fn test() {
        //
    }
}
