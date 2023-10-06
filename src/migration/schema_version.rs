//! [`SchemaVersion`] provides information about versioned database schemas

/// [`SchemaVersion`] provides information about versioned database schemas
#[derive(Debug, Eq, PartialEq)]
pub struct SchemaVersion(u64);

impl<T> From<T> for SchemaVersion
where
    T: Into<u64> + Copy,
{
    fn from(value: T) -> Self {
        Self(value.into())
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
