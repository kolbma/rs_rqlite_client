//! [`SchemaVersion`] provides information about versioned database schemas

/// [`SchemaVersion`] provides information about versioned database schemas
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd)]
pub struct SchemaVersion(pub u64);

impl SchemaVersion {
    /// Checked addition with a signed integer. Computes self + `rhs`, returning `None` if overflow occurred.
    #[inline]
    pub fn checked_add_signed(&self, rhs: i32) -> Option<Self> {
        u64::from(self).checked_add_signed(i64::from(rhs)).map(Self)
    }

    /// Checked integer subtraction. Computes self - `rhs`, returning `None` if overflow occurred.
    #[inline]
    pub fn checked_sub(&self, value: i32) -> Option<Self> {
        if value < 0 {
            u64::from(self).checked_add_signed(-i64::from(value))
        } else {
            // unwraps for sure in this condition
            u64::from(self).checked_sub(u64::try_from(value).unwrap_or_default())
        }
        .map(Self)
    }
}

impl From<&SchemaVersion> for u64 {
    fn from(value: &SchemaVersion) -> Self {
        value.0
    }
}

impl From<SchemaVersion> for u64 {
    fn from(value: SchemaVersion) -> Self {
        value.0
    }
}

impl std::fmt::Display for SchemaVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl std::ops::Sub for SchemaVersion {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Sub<i32> for SchemaVersion {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Self(self.0 - u64::try_from(rhs).expect("unsigned conversion fail"))
    }
}

impl std::ops::SubAssign for SchemaVersion {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl std::ops::SubAssign<i32> for SchemaVersion {
    fn sub_assign(&mut self, rhs: i32) {
        self.0 -= u64::try_from(rhs).expect("unsigned conversion fail");
    }
}

impl std::ops::Add for SchemaVersion {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Add<i32> for SchemaVersion {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self(self.0 + u64::try_from(rhs).expect("unsigned conversion fail"))
    }
}

impl std::ops::AddAssign for SchemaVersion {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::AddAssign<i32> for SchemaVersion {
    fn add_assign(&mut self, rhs: i32) {
        self.0 += u64::try_from(rhs).expect("unsigned conversion fail");
    }
}
