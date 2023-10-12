//! [`SchemaVersion`] provides information about versioned database schemas

/// [`SchemaVersion`] provides information about versioned database schemas
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd)]
pub struct SchemaVersion(pub u64);

/// Maximum [`SchemaVersion`]
pub const MAX: SchemaVersion = SchemaVersion(u64::MAX);

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

    /// Maximum [`SchemaVersion`]
    #[must_use]
    #[inline]
    pub const fn max() -> SchemaVersion {
        MAX
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

impl From<u64> for SchemaVersion {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<i32> for SchemaVersion {
    fn from(value: i32) -> Self {
        if value < 0 {
            Self(0_u64)
        } else {
            0_u64
                .checked_add_signed(i64::from(value))
                .expect("unsigned conversion fail")
                .into()
        }
    }
}

impl From<&SchemaVersion> for usize {
    fn from(value: &SchemaVersion) -> Self {
        Self::try_from(u64::from(value)).unwrap_or(Self::MAX)
    }
}

impl From<SchemaVersion> for usize {
    fn from(value: SchemaVersion) -> Self {
        Self::try_from(u64::from(value)).unwrap_or(Self::MAX)
    }
}

impl From<usize> for SchemaVersion {
    fn from(value: usize) -> Self {
        Self(u64::try_from(value).unwrap_or(u64::MAX))
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

#[cfg(test)]
mod tests {
    use super::SchemaVersion;

    #[test]
    fn from_i32_test() {
        assert_eq!(SchemaVersion::from(i32::MIN), 0.into());
        assert_eq!(SchemaVersion::from(0), 0.into());
        assert_eq!(SchemaVersion::from(i32::MAX), SchemaVersion(2_147_483_647));
    }
}
