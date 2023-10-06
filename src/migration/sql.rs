//! `sql`

use std::{
    path::Path,
    str::{FromStr, Lines},
};

use super::MigrationError;
use crate::Value;

/// `Sql`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sql<'a> {
    sql: &'a str,
    sql_string: Option<String>,
}

impl Sql<'_> {
    #[inline]
    pub fn as_str(&self) -> &'_ str {
        if let Some(sql) = &self.sql_string {
            sql
        } else {
            self.sql
        }
    }

    #[inline]
    pub fn lines(&self) -> Lines {
        self.as_str().lines()
    }
}

impl<'a> From<&'a str> for Sql<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            sql: value,
            sql_string: None,
        }
    }
}

impl<'a> From<&'a Sql<'a>> for &'a str {
    fn from(value: &'a Sql<'a>) -> &'a str {
        value.as_str()
    }
}

impl<'a> TryFrom<Sql<'a>> for Value {
    type Error = serde_json::Error;

    fn try_from(value: Sql<'a>) -> Result<Self, Self::Error> {
        Value::from_str(value.as_str())
    }
}

impl<'a> TryFrom<&Path> for Sql<'a> {
    type Error = MigrationError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let sql_string =
            Some(std::fs::read_to_string(path).map_err(|_err| MigrationError::NoData)?);

        Ok(Self {
            sql: "",
            sql_string,
        })
    }
}

#[cfg(feature = "migration_embed")]
impl<'a> TryFrom<std::borrow::Cow<'a, [u8]>> for Sql<'a> {
    type Error = MigrationError;

    fn try_from(cow: std::borrow::Cow<'a, [u8]>) -> Result<Self, Self::Error> {
        let sql_string =
            Some(String::from_utf8(cow.into_owned()).map_err(|_err| MigrationError::NoData)?);

        Ok(Self {
            sql: "",
            sql_string,
        })
    }
}
